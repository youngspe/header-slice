use crate::header_slice::HeaderSlice;
use crate::utils::{self, Pair};
use alloc::alloc::{alloc, dealloc, realloc, Layout};
use alloc::boxed::Box;
use core::fmt::{self, Debug};
use core::iter;
use core::mem::{self, ManuallyDrop, MaybeUninit};
use core::ops::{Add, AddAssign};
use core::ops::{Deref, DerefMut};
use core::ptr::{self, NonNull};

pub struct HeaderVec<H, T> {
    ptr: NonNull<Pair<H, MaybeUninit<T>>>,
    len: usize,
    cap: usize,
}

const MIN_CAP: usize = 8;

impl<H, T> HeaderVec<H, T> {
    /// The total reserved capacity of the vector.
    pub fn capacity(&self) -> usize {
        if mem::size_of::<T>() == 0 {
            usize::MAX
        } else {
            self.cap
        }
    }

    /// Returns a pointer to a `HeaderSlice` representing this vector.
    pub fn as_ptr(&self) -> NonNull<HeaderSlice<H, T>> {
        utils::pair_as_slice_ptr(self.ptr.cast::<Pair<H, T>>(), self.len)
    }

    /// Returns the raw parts (ptr, length, capacity) of the vector without consuming it.
    /// Use at your own risk: it is possible to create multiple instances of the same vector by
    /// passing this to `from_raw_parts`. Having multiple instances with the same pointer is "safe"
    /// as long as it is never used mutably (or dropped/consumed) as long as more than one instance
    /// exists.
    pub fn as_raw_parts(&mut self) -> (NonNull<Pair<H, MaybeUninit<T>>>, usize, usize) {
        (self.ptr, self.len, self.cap)
    }

    /// Returns the raw parts (ptr, length, capacity) of the vector.
    /// Reconstruct the vector by passing these values to `from_raw_parts`.
    pub fn into_raw_parts(mut self) -> (NonNull<Pair<H, MaybeUninit<T>>>, usize, usize) {
        let parts = self.as_raw_parts();
        mem::forget(self);
        parts
    }

    /// Constructs an instance of this struct using the raw parts returned from `as_raw_parts` or
    /// `into_raw_parts`.
    pub unsafe fn from_raw_parts(
        ptr: NonNull<Pair<H, MaybeUninit<T>>>,
        len: usize,
        cap: usize,
    ) -> Self {
        Self { ptr, len, cap }
    }

    /// Convert `ptr` to a mutable reference to a HeaderSlice with the entire capacity of the vector.
    fn inner_mut(&mut self) -> &mut HeaderSlice<H, MaybeUninit<T>> {
        let ptr = utils::pair_as_slice_ptr(self.ptr, self.cap);
        unsafe { &mut *ptr.as_ptr() }
    }

    /// Returns the `Layout` to be used when allocating the specified capacity.
    fn get_layout(cap: usize) -> Layout {
        let head_layout = Layout::new::<H>();
        let buf_layout = Layout::array::<T>(cap).unwrap();
        head_layout.extend(buf_layout).unwrap().0.pad_to_align()
    }

    /// Reallocate so that the vector has the exact requested capacity
    /// unsafe because the new capacity may be less than self.len
    unsafe fn realloc_exact(&mut self, count: usize) {
        if count == self.cap {
            return;
        }
        let old_layout = Self::get_layout(self.cap);
        let new_layout = Self::get_layout(count);
        let bytes_ptr = realloc(self.ptr.as_ptr() as *mut u8, old_layout, new_layout.size());
        let ptr = utils::set_ptr_value_mut(self.ptr.as_ptr(), bytes_ptr);
        self.ptr = NonNull::new(ptr).unwrap();
        self.cap = count;
    }

    /// Increase capacity so that about half the capacity is unused.
    fn grow(&mut self, target_len: usize) {
        let target_cap = (target_len * 2).max(self.cap);
        unsafe { self.realloc_exact(target_cap) }
    }

    /// Decrease capacity so that about half the capacity is unused.
    /// unsafe because the new capacity may be less than self.len
    unsafe fn shrink(&mut self, target_len: usize) {
        let target_cap = (target_len * 2).max(MIN_CAP).min(self.cap);
        self.realloc_exact(target_cap);
    }

    /// Reallocates if necessary to hold a vector of the given length
    /// unsafe because the new capacity may be less than self.len
    unsafe fn realloc_for(&mut self, len: usize) {
        if len < self.len {
            self.shrink(len);
        } else if len > self.cap {
            self.grow(len);
        }
    }

    /// Push a value to the end of the vector.
    pub fn push(&mut self, val: T) {
        let new_len = self.len + 1;
        if new_len > self.cap {
            self.grow(new_len);
        }
        let index = self.len;
        self.inner_mut().body[index] = MaybeUninit::new(val);
        self.len = new_len;
    }

    /// Pop a value from the end of the vec, if there is one.
    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            return None;
        }
        let new_len = self.len - 1;
        let val = unsafe { ptr::read(self.inner_mut().body[new_len].as_ptr()) };
        unsafe { self.shrink(new_len) };
        self.len = new_len;
        Some(val)
    }

    /// Removes a value at the given index, if it exiss.
    /// All entries after `index` will be shifted to the left.
    pub fn remove(&mut self, index: usize) -> Option<T> {
        if index >= self.len {
            return None;
        }
        let target_ptr = &mut self.inner_mut().body[index] as *mut MaybeUninit<T>;
        let val = unsafe { ptr::read(target_ptr) };
        let copy_len = self.len - index - 1;
        let copy_src = unsafe { target_ptr.add(1) };
        unsafe { ptr::copy(copy_src, target_ptr, copy_len) };
        unsafe { self.shrink(self.len - 1) };
        self.len -= 1;
        Some(unsafe { val.assume_init() })
    }

    /// Remove an element at `index` if it exists by replacing it with the last
    /// element of the vector.
    pub fn swap_remove(&mut self, index: usize) -> Option<T> {
        if index >= self.len {
            return None;
        }

        // pop can't fail -- since index is in [0, len), len must be at least one
        let last = self.pop().unwrap();

        if index == self.len {
            return Some(last);
        }

        Some(mem::replace(&mut self.body[index], last))
    }

    /// Inserts an element at `index`, shifting all elements after `index` to
    /// the right.
    /// Panics if `index > self.len()`
    pub fn insert(&mut self, index: usize, val: T) {
        assert!(index <= self.len);
        if index == self.len {
            self.push(val);
            return;
        }
        // let target_ptr = &mut self.inner_mut().body[index] as *mut MaybeUninit<T>;
        let target_ptr = unsafe { self.inner_mut().body.as_mut_ptr().add(index) };
        let copy_len = self.len - index;
        let copy_dest = unsafe { target_ptr.add(1) };
        unsafe { ptr::copy(target_ptr, copy_dest, copy_len) };
        unsafe {
            ptr::write(target_ptr, MaybeUninit::new(val));
        };
        self.grow(self.len);
        self.len += 1;
    }

    /// Creates an empty `HeaderVec` with the specified capacity.
    pub fn with_capacity(head: H, cap: usize) -> Self {
        let layout = Self::get_layout(cap);
        let bytes_ptr = unsafe { alloc(layout) };
        let mut ptr = NonNull::new(bytes_ptr as *mut Pair<H, MaybeUninit<T>>).unwrap();
        unsafe { ptr::write(&mut ptr.as_mut().0 as *mut H, head) }
        Self { ptr, len: 0, cap }
    }

    /// Creates an empty `HeaderVec`.
    pub fn new(head: H) -> Self {
        Self::with_capacity(head, MIN_CAP)
    }

    /// Shortens the vector to the given length.
    /// Panics if `new_len > self.len()`.
    pub fn truncate(&mut self, new_len: usize) {
        assert!(new_len <= self.len);
        if new_len == self.len {
            return;
        }

        for index in new_len..self.len {
            unsafe { self.drop_item(index) }
        }
        unsafe { self.shrink(new_len) };
        self.len = new_len;
    }

    /// Resizes the vector.
    /// If `new_len > self.len()`, the elements will be instantiated with the
    /// given function.
    pub fn resize_with(&mut self, new_len: usize, mut f: impl FnMut() -> T) {
        if new_len < self.len {
            self.truncate(new_len);
        } else {
            for _ in self.len..new_len {
                self.push(f());
            }
        }
    }

    /// Drops the element at the given index.
    unsafe fn drop_item(&mut self, index: usize) {
        let ptr = &mut self.inner_mut().body[index] as *mut _ as *mut T;
        ptr::drop_in_place(ptr);
    }

    /// Creates a new instance of `HeaderVec` from the given header and iterator.
    pub fn from_iter<I: IntoIterator<Item = T>>(head: H, iter: I) -> Self {
        let iter = iter.into_iter();
        let (lower, _) = iter.size_hint();
        let mut this = Self::with_capacity(head, lower);
        this.extend(iter);
        this
    }

    /// Reallocates so there is no excess capacity (i.e. capacity == length).
    pub fn shrink_to_fit(&mut self) {
        unsafe { self.realloc_exact(self.len) }
    }

    /// Converts the vector into a boxed `HeaderSlice`.
    pub fn into_box(mut self) -> Box<HeaderSlice<H, T>> {
        self.shrink_to_fit();
        let b = unsafe { Box::from_raw(self.as_ptr().as_ptr()) };
        mem::forget(self);
        b
    }

    /// Creates a vector from a boxed `HeaderSlice`.
    pub fn from_box(src: Box<HeaderSlice<H, T>>) -> Self {
        let len = src.body.len();
        let ptr = NonNull::new(Box::into_raw(src) as *mut Pair<H, MaybeUninit<T>>).unwrap();
        Self { ptr, len, cap: len }
    }

    /// Reserve enough capacity to add at least `additional` elements without realllocating.
    pub fn reserve(&mut self, additional: usize) {
        unsafe { self.realloc_for(self.len + additional) };
    }

    /// Reserve enough capacity to add  exactly `additional` elements without realllocating.
    pub fn reserve_exact(&mut self, additional: usize) {
        let new_cap = self.len + additional;
        if new_cap <= self.cap {
            return;
        }
        unsafe { self.realloc_exact(new_cap) };
    }

    /// Deallocates the vector. Do not use the pointer after this.
    unsafe fn dealloc(&mut self) {
        dealloc(self.ptr.as_ptr() as *mut u8, Self::get_layout(self.cap));
    }

    /// Consumes the vector and returns an iterator of its values.
    pub fn into_values(self) -> IntoValuesIter<H, T> {
        IntoValuesIter {
            inner: ManuallyDrop::new(self),
            index: 0,
        }
    }

    /// Delete all items in the vector and reallocate so there is no excess capacity.
    pub fn clear(&mut self) {
        self.clear_in_place();
        unsafe { self.realloc_exact(0) }
    }

    /// Delete all items in the vector without reallocating.
    pub fn clear_in_place(&mut self) {
        unsafe {
            for i in 0..self.len {
                self.drop_item(i);
            }
            self.len = 0;
        }
    }

    /// Copies the contents of a slice into a new `HeaderVec`.
    /// Do not use or drop the contents of the original slice after this.
    pub unsafe fn copy_from_ptr_unsafe(head: H, src: *mut T, len: usize) -> Self {
        let mut this = Self::with_capacity(head, len);
        let dest = this.body.as_mut_ptr();
        ptr::copy_nonoverlapping(src, dest, len);
        this.len = len;
        this
    }
}

impl<H, T: Copy> HeaderVec<H, T> {
    /// Copies the contents of a slice into a new `HeaderVec`.
    pub fn copy_from_slice(head: H, src: &[T]) -> Self {
        unsafe { Self::copy_from_ptr_unsafe(head, src.as_ptr() as *mut T, src.len()) }
    }

    /// Copies the contents onto the end of the vector.
    pub fn extend_from_slice(&mut self, src: &[T]) {
        let new_len = self.len + src.len();
        if new_len > self.cap {
            self.grow(new_len);
        }
        let old_len = self.len;
        let uninit_slice = &mut self.inner_mut().body[old_len..];
        unsafe {
            ptr::copy(
                src.as_ptr() as *mut MaybeUninit<T>,
                uninit_slice.as_mut_ptr(),
                src.len(),
            )
        }
        self.len = new_len;
    }
}

impl<H, T: Clone> HeaderVec<H, T> {
    /// Resize the vector. If `new_len > self.len()`, new entries will be cloned
    /// from `val`.
    pub fn resize(&mut self, new_len: usize, mut val: T) {
        if new_len < self.len {
            self.truncate(new_len);
        } else if new_len > self.len {
            for _ in self.len..new_len - 1 {
                let next_val = val.clone();
                self.push(val);
                val = next_val;
            }
            self.push(val);
        }
    }
}

impl<H, T: Default> HeaderVec<H, T> {
    /// Resize the vector. If `new_len > self.len()`, new entries will use the
    /// default value of `T`.
    pub fn resize_default(&mut self, new_len: usize) {
        self.resize_with(new_len, Default::default)
    }
}

impl<H, T> Deref for HeaderVec<H, T> {
    type Target = HeaderSlice<H, T>;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.as_ptr().as_ptr() }
    }
}

impl<H, T> DerefMut for HeaderVec<H, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.as_ptr().as_ptr() }
    }
}

impl<H, T> Drop for HeaderVec<H, T> {
    fn drop(&mut self) {
        unsafe {
            for i in 0..self.len {
                self.drop_item(i);
            }
            self.dealloc();
        }
    }
}

impl<H: Clone, T: Clone> Clone for HeaderVec<H, T> {
    fn clone(&self) -> Self {
        Self::from_iter(self.head.clone(), self.body.iter().cloned())
    }
}

impl<H, T> Extend<T> for HeaderVec<H, T> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        for x in iter {
            self.push(x);
        }
    }
}

impl<H, T, I: IntoIterator<Item = T>> AddAssign<I> for HeaderVec<H, T> {
    fn add_assign(&mut self, rhs: I) {
        self.extend(rhs);
    }
}

impl<H, T, I: IntoIterator<Item = T>> Add<I> for HeaderVec<H, T> {
    type Output = Self;
    fn add(mut self, rhs: I) -> Self {
        self += rhs;
        self
    }
}

impl<H: Debug, T: Debug> Debug for HeaderVec<H, T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let hslice: &HeaderSlice<H, T> = self.deref();
        hslice.fmt(f)
    }
}

impl<H: Default, T> iter::FromIterator<T> for HeaderVec<H, T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Self::from_iter(H::default(), iter)
    }
}

pub struct IntoValuesIter<H, T> {
    inner: ManuallyDrop<HeaderVec<H, T>>,
    index: usize,
}

impl<H, T> Iterator for IntoValuesIter<H, T> {
    type Item = T;
    fn next(&mut self) -> Option<T> {
        if self.index >= self.inner.len() {
            return None;
        }

        let val = unsafe { ptr::read(self.inner.inner_mut().body[self.index].as_ptr()) };
        self.index += 1;

        if self.index == self.inner.len() {
            unsafe { self.inner.dealloc() };
        }

        Some(val)
    }
}

/// Creates a `HeaderVec` with the given header and elements;
///
/// ## Examples:
/// - `header_vec!["foo"; 1, 2, 3]`
/// - `header_vec![123; true; 32]`
#[macro_export]
macro_rules! header_vec {
    // Take a list of elements:
    ($h:expr; $($v:expr),* $(,)?) => {{
        let mut src = [$($v),*];
        let v = unsafe {
            $crate::pair::HeaderVec::copy_from_ptr_unsafe($h, src.as_mut_ptr(), src.len())
        };
        core::mem::forget(src);
        v
    }};
    // Take a cloneable element and desired length:
    ($h:expr; $v:expr; $len:expr) => {{
        let mut v = $crate::pair::HeaderVec::with_capacity($len);
        v.resize($len, $v);
        v
    }};
}
