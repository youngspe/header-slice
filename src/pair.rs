use crate::slice::HeaderSlice;
use core::ptr::{self, NonNull};

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
