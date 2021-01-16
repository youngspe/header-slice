#![no_std]
extern crate alloc;

#[macro_use]
mod utils;

pub mod pair;
pub mod slice;
#[cfg(test)]
mod test;
pub mod vec;

pub use slice::HeaderSlice;
pub use vec::HeaderVec;
