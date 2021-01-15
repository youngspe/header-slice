use crate::header_vec;
use alloc::boxed::Box;

#[test]
fn add_range() {
    assert_eq!(
        header_vec!["foo"; 4, 5] + (6..=9),
        header_vec!["foo"; 4, 5, 6, 7, 8, 9],
    );
}

#[test]
fn add_array() {
    assert_eq!(
        header_vec!["foo"; 1, 2] + [3, 4, 5].iter().copied(),
        header_vec!["foo"; 1, 2, 3, 4, 5],
    );
}

#[test]
fn add_vec() {
    let x: Box<[i32]> = Box::new([3, 4, 5]);
    assert_eq!(
        header_vec!["foo"; 1, 2] + x.into_vec(),
        header_vec!["foo"; 1, 2, 3, 4, 5],
    );
}

#[test]
fn add_assign_range() {
    let mut v = header_vec!["foo"; 4, 5];
    v += 6..=9;

    assert_eq!(v, header_vec!["foo"; 4, 5, 6, 7, 8, 9]);
}

#[test]
fn add_assign_iter() {
    let mut v = header_vec!["foo"; 1, 2];
    v += [3, 4, 5].iter().copied();
    assert_eq!(v, header_vec!["foo"; 1, 2, 3, 4, 5]);
}

#[test]
fn add_assign_vec() {
    let mut v = header_vec!["foo"; 1, 2];
    let x: Box<[i32]> = Box::new([3, 4, 5]);
    v += x.into_vec();
    assert_eq!(v, header_vec!["foo"; 1, 2, 3, 4, 5]);
}

#[test]
fn extend() {
    let mut v = header_vec!["foo"; 1, 2];
    v.extend(1..=3);
    assert_eq!(v, header_vec!["foo"; 1, 2, 1, 2, 3]);
}


#[test]
fn extend_from_slice() {
    let mut v = header_vec!["foo"; 1, 2, 3];
    v.extend_from_slice(&[4, 5, 6]);
    assert_eq!(v, header_vec!["foo"; 1, 2, 3, 4, 5, 6]);
}
