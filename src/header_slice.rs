use core::fmt::{self, Debug};
use core::ptr;

#[repr(C)]
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
