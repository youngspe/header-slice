#![no_std]
extern crate alloc;

pub mod header_slice;
pub mod header_vec;
mod utils;

pub use header_slice::HeaderSlice;
pub use header_vec::HeaderVec;
