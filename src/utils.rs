use crate::slice::HeaderSlice;
use core::ptr::{self, NonNull};

macro_rules! partial_ord_chain {
    ($($lhs:expr => $rhs:expr),* $(,)?) => {{
        $({
            let ordering = $lhs.partial_cmp(&$rhs)?;
            if ordering != core::cmp::Ordering::Equal { return Some(ordering); }
        })*
        Some(core::cmp::Ordering::Equal)
    }};
}

macro_rules! ord_chain {
    ($($lhs:expr => $rhs:expr),* $(,)?) => {{
        $({
            let ordering = $lhs.cmp(&$rhs);
            if ordering != core::cmp::Ordering::Equal { return ordering; }
        })*
        core::cmp::Ordering::Equal
    }};
}

/// stand-in for the unstablem set_ptr_value feature
pub fn set_ptr_value<T: ?Sized>(mut ptr: *const T, value: *const u8) -> *const T {
    // obtain a pointer to the variable 'ptr':
    let ptr_ptr: *mut *const T = &mut ptr as *mut *const T;
    // convert this to a pointer to a u8 pointer.
    // this means, regardless of whether T is sized, this points to a thin ptr.
    let thin_ptr_ptr = ptr_ptr as *mut *const u8;

    unsafe { ptr::write(thin_ptr_ptr, value) }
    ptr
}

/// stand-in for the unstablem set_ptr_value feature
pub fn set_ptr_value_mut<T: ?Sized>(ptr: *mut T, value: *mut u8) -> *mut T {
    set_ptr_value(ptr, value) as *mut T
}

#[repr(C)]
pub struct Pair<A, B: ?Sized>(pub A, pub B);

pub fn pair_as_slice_ptr<H, T>(
    pair: NonNull<Pair<H, T>>,
    len: usize,
) -> NonNull<HeaderSlice<H, T>> {
    let slice_ptr = ptr::slice_from_raw_parts_mut(pair.as_ptr(), len);
    let hslice = slice_ptr as *mut HeaderSlice<H, T>;
    unsafe { NonNull::new_unchecked(hslice) }
}
