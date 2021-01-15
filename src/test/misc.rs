use crate::{header_vec, HeaderVec};
use alloc::vec::Vec;

#[test]
fn reserve() {
    let mut v = header_vec!["foo"; 1, 2, 3, 4];
    v.reserve(6);
    assert!(v.capacity() >= 10);
    assert_eq!(v, header_vec!["foo"; 1, 2, 3, 4]);
}

#[test]
fn reserve_exact() {
    let mut v = header_vec!["foo"; 1, 2, 3, 4];
    v.reserve_exact(6);
    assert_eq!(v.capacity(), 10);
    assert_eq!(v, header_vec!["foo"; 1, 2, 3, 4]);
}

#[test]
fn zst_capacity() {
    let v = header_vec!["foo"; (), (), ()];
    assert_eq!(v.capacity(), usize::MAX);
}

#[test]
fn into_raw_parts() {
    let v1 = header_vec!["foo"; 1, 2, 3];
    let (ptr, len, cap) = v1.into_raw_parts();
    let v2 = unsafe { HeaderVec::from_raw_parts(ptr, len, cap) };
    assert_eq!(v2, header_vec!["foo"; 1, 2, 3]);
}

#[test]
fn as_raw_parts() {
    let mut v1 = header_vec!["foo"; 1, 2, 3];
    let (ptr, len, cap) = v1.as_raw_parts();
    // leak v1 so we don't drop twice or have two instances at once
    core::mem::forget(v1);
    let v2 = unsafe { HeaderVec::from_raw_parts(ptr, len, cap) };
    assert_eq!(v2, header_vec!["foo"; 1, 2, 3]);
}

#[test]
fn shrink_to_fit() {
    let mut v = header_vec!["foo"; 1, 2, 3, 4];
    v.reserve_exact(6);
    assert_eq!(v.capacity(), 10);
    v.shrink_to_fit();
    assert_eq!(v.capacity(), 4);
    assert_eq!(v, header_vec!["foo"; 1, 2, 3, 4]);
}

#[test]
fn into_header_values() {
    let v = header_vec!["foo"; 1, 2, 3, 4];
    let (head, vals) = v.into_header_values();
    assert_eq!(head, "foo");
    assert_eq!(&vals.collect::<Vec<_>>(), &[1, 2, 3, 4]);
}

#[test]
fn into_values() {
    let v = header_vec!["foo"; 1, 2, 3, 4];
    let vals = v.into_values();
    assert_eq!(&vals.collect::<Vec<_>>(), &[1, 2, 3, 4]);
}
