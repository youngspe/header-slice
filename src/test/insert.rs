use crate::{header_vec, HeaderVec};

#[test]
fn push() {
    let mut v1 = header_vec!["foo"; 0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    for i in 10..128 {
        v1.push(i);
    }
    let v2 = HeaderVec::from_iter("foo", 0..128);
    assert_eq!(v1, v2);
}

#[test]
fn push_to_empty() {
    let mut v1 = HeaderVec::new("foo");
    for i in 0..128 {
        v1.push(i);
    }
    let v2 = HeaderVec::from_iter("foo", 0..128);
    assert_eq!(v1, v2);
}

#[test]
fn push_zst() {
    fn inner_test<H: Copy + core::fmt::Debug + Eq>(h: H) {
        let mut v = header_vec![h;];
        for i in 0..128 {
            v.push(());
            assert_eq!(v, header_vec![h; (); i + 1]);
        }
    }

    // with non-zero-sized header
    inner_test([1, 2, 3, 4, 5]);
    // with zero-sized header
    inner_test(());
}

#[test]
fn insert() {
    let mut v = header_vec!["foo"; 1];
    v.insert(0, 123);
    assert_eq!(v, header_vec!["foo"; 123, 1]);

    let mut v = header_vec!["foo"; 1, 2, 3, 4, 5, 6];
    v.insert(2, 123);
    assert_eq!(v, header_vec!["foo"; 1, 2, 123, 3, 4, 5, 6]);

    let mut v = header_vec!["foo"; 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
    v.insert(9, 123);
    assert_eq!(
        v,
        header_vec!["foo"; 1, 2, 3, 4, 5, 6, 7, 8, 9, 123, 10, 11, 12, 13, 14, 15, 16]
    );
}

#[test]
fn insert_to_empty() {
    let mut v = HeaderVec::new("foo");
    v.insert(0, 1);
    assert_eq!(v, header_vec!["foo"; 1]);
}

#[test]
fn insert_to_end() {
    let mut v = header_vec!["foo"; 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
    v.insert(16, 123);
    assert_eq!(
        v,
        header_vec!["foo"; 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 123]
    );
}

#[test]
#[should_panic]
fn insert_out_of_bounds() {
    let mut v = header_vec!["foo"; 1, 2, 3];
    v.insert(4, 123);
}

#[test]
fn insert_sorted() {
    let mut v = header_vec!["foo"; 1, 3, 5, 7, 9];
    v.insert_sorted(6);
    assert_eq!(v, header_vec!["foo"; 1, 3, 5, 6, 7, 9]);
    v.insert_sorted(15);
    assert_eq!(v, header_vec!["foo"; 1, 3, 5, 6, 7, 9, 15]);
    v.insert_sorted(3);
    assert_eq!(v, header_vec!["foo"; 1, 3, 3, 5, 6, 7, 9, 15]);
    v.insert_sorted(-20);
    assert_eq!(v, header_vec!["foo"; -20, 1, 3, 3, 5, 6, 7, 9, 15]);
    v.insert_sorted(2);
    assert_eq!(v, header_vec!["foo"; -20, 1, 2, 3, 3, 5, 6, 7, 9, 15]);
}
#[test]
fn insert_sorted_empty() {
    let mut v = HeaderVec::new("foo");
    v.insert_sorted(3);
    assert_eq!(v, header_vec!["foo"; 3]);
    v.insert_sorted(1);
    assert_eq!(v, header_vec!["foo"; 1, 3]);
    v.insert_sorted(9);
    assert_eq!(v, header_vec!["foo"; 1, 3, 9]);
}

#[test]
fn insert_or_replace_sorted() {
    let mut v = header_vec!["foo"; 1, 3, 5, 7, 9];
    assert_eq!(v.insert_or_replace_sorted(6), None);
    assert_eq!(v, header_vec!["foo"; 1, 3, 5, 6, 7, 9]);
    assert_eq!(v.insert_or_replace_sorted(3), Some(3));
    assert_eq!(v, header_vec!["foo"; 1, 3, 5, 6, 7, 9]);
}

#[test]
fn insert_zst() {
    fn inner_test<H: Copy + core::fmt::Debug + Eq>(h: H) {
        let mut v = header_vec![h;];
        v.insert(0, ());
        assert_eq!(v, header_vec![h; (); 1]);
        v.insert(1, ());
        assert_eq!(v, header_vec![h; (); 2]);
        v.insert(0, ());
        assert_eq!(v, header_vec![h; (); 3]);
        v.insert(3, ());
        assert_eq!(v, header_vec![h; (); 4]);
        v.insert(2, ());
        assert_eq!(v, header_vec![h; (); 5]);
        v.insert(2, ());
        assert_eq!(v, header_vec![h; (); 6]);
    }

    // with non-zero-sized header
    inner_test([1, 2, 3, 4, 5]);
    // with zero-sized header
    inner_test(());
}
