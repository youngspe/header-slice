use crate::{header_vec, HeaderVec};

#[test]
fn truncate() {
    let mut v = header_vec!["foo"; 1, 2, 3, 4];
    v.truncate(2);
    assert_eq!(v, header_vec!["foo"; 1, 2]);
}

#[test]
fn truncate_zero() {
    let mut v = header_vec!["foo"; 1, 2, 3, 4];
    v.truncate(0);
    assert_eq!(v, header_vec!["foo";]);
}

#[test]
#[should_panic]
fn truncate_larger() {
    let mut v = header_vec!["foo"; 1, 2, 3];
    v.truncate(4);
}

#[test]
fn truncate_unchanged() {
    let mut v = header_vec!["foo"; 1, 2, 3];
    v.truncate(3);
    assert_eq!(v, header_vec!["foo"; 1, 2, 3]);
}

#[test]
fn resize_from_empty() {
    let mut v = HeaderVec::new("foo");
    v.resize(4, 9);
    assert_eq!(v, header_vec!["foo"; 9, 9, 9, 9]);
}

#[test]
fn resize_to_empty() {
    let mut v = header_vec!["foo"; 1, 2, 3, 4];
    v.resize(0, 9);
    assert_eq!(v, header_vec!["foo";]);
}

#[test]
fn resize_smaller() {
    let mut v = header_vec!["foo"; 1, 2, 3, 4];
    v.resize(2, 9);
    assert_eq!(v, header_vec!["foo"; 1, 2]);
}

#[test]
fn resize_with_smaller() {
    let mut v = header_vec!["foo"; 1, 2, 3, 4];
    v.resize_with(2, || panic!());
    assert_eq!(v, header_vec!["foo"; 1, 2]);
}

#[test]
fn resize_default_smaller() {
    let mut v = header_vec!["foo"; 1, 2, 3, 4];
    v.resize_default(2);
    assert_eq!(v, header_vec!["foo"; 1, 2]);
}

#[test]
fn resize_larger() {
    let mut v = header_vec!["foo"; 1, 2, 3, 4];
    v.resize(6, -1);
    assert_eq!(v, header_vec!["foo"; 1, 2, 3, 4, -1, -1]);
}

#[test]
fn resize_with_larger() {
    let mut v = header_vec!["foo"; 1, 2, 3, 4];
    v.resize_with(6, || -1);
    assert_eq!(v, header_vec!["foo"; 1, 2, 3, 4, -1, -1]);
}

#[test]
fn resize_default_larger() {
    let mut v = header_vec!["foo"; 1, 2, 3, 4];
    v.resize_default(6);
    assert_eq!(v, header_vec!["foo"; 1, 2, 3, 4, 0, 0]);
}

#[test]
fn clear() {
    let mut v = header_vec!["foo"; 7; 100];
    v.clear();
    assert_eq!(v, header_vec!["foo";]);
}

#[test]
fn clear_in_place() {
    let mut v = header_vec!["foo"; 7; 100];
    let old_cap = v.capacity();
    v.clear_in_place();
    assert_eq!(v, header_vec!["foo";]);
    assert_eq!(v.capacity(), old_cap);
}
