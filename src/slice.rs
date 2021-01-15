use crate::HeaderVec;
use alloc::alloc::Layout;
use alloc::borrow::{Borrow, ToOwned};
use core::cmp::Ordering;
use core::fmt::{self, Debug};
use core::ptr;

#[repr(C)]
#[derive(Hash)]
pub struct HeaderSlice<H, T> {
    pub head: H,
    pub body: [T],
}

impl<H, T> HeaderSlice<H, T> {
    fn resize_ptr(this: *mut Self, len: usize) -> *mut Self {
        let slice_ptr = ptr::slice_from_raw_parts_mut(this as *mut T, len);
        slice_ptr as *mut Self
    }

    pub fn as_truncated(&self, len: usize) -> &Self {
        assert!(len <= self.body.len());
        let ptr = Self::resize_ptr(self as *const _ as *mut Self, len);
        unsafe { &*ptr }
    }

    pub fn as_truncated_mut(&mut self, len: usize) -> &mut Self {
        assert!(len <= self.body.len());
        let ptr = Self::resize_ptr(self as *mut _, len);
        unsafe { &mut *ptr }
    }

    pub unsafe fn resized_unchecked(&mut self, len: usize) -> &mut Self {
        let ptr = Self::resize_ptr(self as *mut _, len);
        &mut *ptr
    }

    pub fn len(&self) -> usize {
        self.body.len()
    }

    /// Returns the memory layout for an instance with the given length
    pub fn layout_for_len(len: usize) -> Layout {
        let head_layout = Layout::new::<H>();
        let buf_layout = Layout::array::<T>(len).unwrap();
        head_layout.extend(buf_layout).unwrap().0.pad_to_align()
    }
}

impl<H: Clone, T: Clone> ToOwned for HeaderSlice<H, T> {
    type Owned = HeaderVec<H, T>;
    fn to_owned(&self) -> Self::Owned {
        HeaderVec::from_iter(self.head.clone(), self.body.iter().cloned())
    }
}

impl<H: Debug, T: Debug> Debug for HeaderSlice<H, T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("[")?;
        self.head.fmt(f)?;
        if self.body.len() == 0 {
            f.write_str(";]")?;
        } else {
            f.write_str("; ")?;
            self.body[0].fmt(f)?;
            for item in &self.body[1..] {
                f.write_str(", ")?;
                item.fmt(f)?;
            }
            f.write_str("]")?;
        }
        Ok(())
    }
}

impl<H, T, Rhs: ?Sized> PartialEq<Rhs> for HeaderSlice<H, T>
where
    H: PartialEq,
    T: PartialEq,
    Rhs: Borrow<HeaderSlice<H, T>>,
{
    fn eq(&self, rhs: &Rhs) -> bool {
        let rhs = rhs.borrow();
        self.head == rhs.head && self.body == rhs.body
    }
}

impl<H: Eq, T: Eq> Eq for HeaderSlice<H, T> {}

impl<H, T, Rhs: ?Sized> PartialOrd<Rhs> for HeaderSlice<H, T>
where
    H: PartialOrd,
    T: PartialOrd,
    Rhs: Borrow<HeaderSlice<H, T>>,
{
    fn partial_cmp(&self, rhs: &Rhs) -> Option<Ordering> {
        let rhs = rhs.borrow();
        partial_ord_chain! {
            self.head => rhs.head,
            self.body => rhs.body,
        }
    }
}

impl<H: Ord, T: Ord> Ord for HeaderSlice<H, T> {
    fn cmp(&self, rhs: &Self) -> Ordering {
        ord_chain! {
            self.head => rhs.head,
            self.body => rhs.body,
        }
    }
}
