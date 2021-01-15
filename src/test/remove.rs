use crate::{header_vec, HeaderVec};

#[test]
fn remove() {
    let mut v = header_vec!["foo"; 1, 2, 3, 4, 5];
    assert_eq!(v.remove(3), Some(4));
    assert_eq!(v, header_vec!["foo"; 1, 2, 3, 5]);
    assert_eq!(v.remove(1), Some(2));
    assert_eq!(v, header_vec!["foo"; 1, 3, 5]);
    assert_eq!(v.remove(2), Some(5));
    assert_eq!(v, header_vec!["foo"; 1, 3]);
    assert_eq!(v.remove(0), Some(1));
    assert_eq!(v, header_vec!["foo"; 3]);
    assert_eq!(v.remove(0), Some(3));
    assert_eq!(v, header_vec!["foo";]);
}

#[test]
fn swap_remove() {
    let mut v = header_vec!["foo"; 1, 2, 3, 4, 5];
    assert_eq!(v.swap_remove(2), Some(3));
    assert_eq!(v, header_vec!["foo"; 1, 2, 5, 4]);
    assert_eq!(v.swap_remove(7), None);
    assert_eq!(v, header_vec!["foo"; 1, 2, 5, 4]);
}

#[test]
fn swap_remove_zst() {
    let mut v = header_vec!["foo"; (); 5];
    assert_eq!(v.swap_remove(2), Some(()));
    assert_eq!(v, header_vec!["foo"; (); 4]);
    assert_eq!(v.swap_remove(7), None);
    assert_eq!(v, header_vec!["foo"; (); 4]);
}

#[test]
fn remove_zst() {
    fn inner_test<H: Copy + core::fmt::Debug + Eq>(h: H) {
        let mut v = header_vec![h; (); 5];
        assert_eq!(v.remove(3), Some(()));
        assert_eq!(v, header_vec![h; (); 4]);
        assert_eq!(v.remove(10), None);
        assert_eq!(v, header_vec![h; (); 4]);
        assert_eq!(v.remove(4), None);
        assert_eq!(v, header_vec![h; (); 4]);
        assert_eq!(v.remove(3), Some(()));
        assert_eq!(v, header_vec![h; (); 3]);
        assert_eq!(v.remove(0), Some(()));
        assert_eq!(v, header_vec![h; (); 2]);
        assert_eq!(v.remove(1), Some(()));
        assert_eq!(v, header_vec![h; (); 1]);
        assert_eq!(v.remove(0), Some(()));
        assert_eq!(v, header_vec![h;]);
    }

    // with non-zero-sized header
    inner_test([1, 2, 3]);
    // with zero-sized header
    inner_test(());
}

#[test]
fn remove_from_empty() {
    let mut v = HeaderVec::<_, i32>::new("foo");
    assert_eq!(v.remove(0), None);
    assert_eq!(v, header_vec!["foo";]);
    assert_eq!(v.remove(1), None);
    assert_eq!(v, header_vec!["foo";]);
    assert_eq!(v.remove(8), None);
    assert_eq!(v, header_vec!["foo";]);
}

#[test]
fn pop() {
    let mut v = header_vec!["foo"; 1, 2, 3, 4, 5];
    assert_eq!(v.pop(), Some(5));
    assert_eq!(v, header_vec!["foo"; 1, 2, 3, 4]);
    assert_eq!(v.pop(), Some(4));
    assert_eq!(v, header_vec!["foo"; 1, 2, 3]);
    assert_eq!(v.pop(), Some(3));
    assert_eq!(v, header_vec!["foo"; 1, 2]);
    assert_eq!(v.pop(), Some(2));
    assert_eq!(v, header_vec!["foo"; 1]);
    assert_eq!(v.pop(), Some(1));
    assert_eq!(v, header_vec!["foo";]);
    assert_eq!(v.pop(), None);
    assert_eq!(v, header_vec!["foo";]);
}

#[test]
fn pop_zst() {
    fn inner_test<H: Copy + core::fmt::Debug + Eq>(h: H) {
        let mut v = header_vec![h; (); 5];
        assert_eq!(v.pop(), Some(()));
        assert_eq!(v, header_vec![h; (); 4]);
        assert_eq!(v.pop(), Some(()));
        assert_eq!(v, header_vec![h; (); 3]);
        assert_eq!(v.pop(), Some(()));
        assert_eq!(v, header_vec![h; (); 2]);
        assert_eq!(v.pop(), Some(()));
        assert_eq!(v, header_vec![h; (); 1]);
        assert_eq!(v.pop(), Some(()));
        assert_eq!(v, header_vec![h; (); 0]);
        assert_eq!(v.pop(), None);
        assert_eq!(v, header_vec![h; (); 0]);
    }

    // with non-zero-sized header
    inner_test([1, 2, 3, 4, 5]);
    // with zero-sized header
    inner_test(());
}
