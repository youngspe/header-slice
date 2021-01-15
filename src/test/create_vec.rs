use crate::{header_vec, HeaderVec};

#[derive(Copy, Clone, Default, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Zst;

#[test]
fn from_macro_exact_cap() {
    let empty: HeaderVec<_, i32> = header_vec!["foo";];
    assert_eq!(empty.capacity(), 0);
    assert_eq!(header_vec!["foo"; 1].capacity(), 1);
    assert_eq!(
        header_vec!["foo"; 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13].capacity(),
        13,
    );
    assert_eq!(header_vec!["foo"; 1, 2, 3].capacity(), 3);

    assert_eq!(header_vec!["foo"; 7; 0].capacity(), 0);
    assert_eq!(header_vec!["foo"; 7; 1].capacity(), 1);
    assert_eq!(header_vec!["foo"; 7; 5].capacity(), 5);
    assert_eq!(header_vec!["foo"; 7; 100].capacity(), 100);
}

#[test]
fn vec_of_zst() {
    let v1: HeaderVec<&str, Zst> = header_vec!["foo"; Zst, Zst, Zst, Zst, Zst];
    assert_eq!(v1.head, "foo");
    assert_eq!(v1.body, [Zst; 5]);
    let v2: HeaderVec<bool, Zst> = header_vec![false; Zst; 7];
    assert_eq!(v2.head, false);
    assert_eq!(v2.body, [Zst; 7]);
}

#[test]
fn vec_zst_head() {
    let v1: HeaderVec<Zst, i32> = header_vec![Zst; 1, 2, 3];
    assert_eq!(v1.head, Zst);
    assert_eq!(v1.body, [1, 2, 3]);
    let v2: HeaderVec<Zst, u16> = header_vec![Zst; 7; 9];
    assert_eq!(v2.head, Zst);
    assert_eq!(v2.body, [7; 9]);
}

#[test]
fn vec_zst_head_and_body() {
    let v1: HeaderVec<Zst, ()> = header_vec![Zst; (), (), (), ()];
    assert_eq!(v1.head, Zst);
    assert_eq!(v1.body, [(); 4]);
    let v2: HeaderVec<(), Zst> = header_vec![(); Zst; 500];
    assert_eq!(v2.head, ());
    assert_eq!(v2.body, [Zst; 500]);
}

#[test]
fn vec_len_correct() {
    assert_eq!(header_vec!["foo"; 1, 2, 3].len(), 3);
    assert_eq!(header_vec!["bar"; true; 20].len(), 20);
    assert_eq!(header_vec!["baz"; (); 1000].len(), 1000);
}

#[test]
fn copy_from_slice() {
    let v = HeaderVec::copy_from_slice("foo", &[1, 2, 3, 4, 5]);
    assert_eq!(v, header_vec!["foo"; 1, 2, 3, 4, 5]);
}

#[test]
fn from_iterator_default_head() {
    let v = [1, 2, 3, 4]
        .iter()
        .copied()
        .collect::<HeaderVec<bool, i32>>();
    assert_eq!(v, header_vec![false; 1, 2, 3, 4]);
}

#[test]
fn with_capacity() {
    let mut v1 = HeaderVec::with_capacity("foo", 10);
    assert_eq!(v1.capacity(), 10);
    assert_eq!(v1, header_vec!["foo";]);
    v1 += 0..10;
    assert_eq!(v1.capacity(), 10);
    assert_eq!(v1, header_vec!["foo"; 0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);

    let mut v2 = HeaderVec::with_capacity("foo", 0);
    assert_eq!(v2.capacity(), 0);
    assert_eq!(v2, header_vec!["foo";]);
    v2 += 0..5;
    assert_eq!(v2, header_vec!["foo"; 0, 1, 2, 3, 4]);
}

#[test]
fn clone() {
    let v = header_vec!["foo"; 1, 2, 3];
    assert_eq!(v.clone(), v);
}

#[test]
fn clone_empty() {
    let v = HeaderVec::<_, i32>::new("foo");
    assert_eq!(v.clone(), v);
}

#[test]
fn clone_zst() {
    let v = header_vec!["foo"; (); 5];
    assert_eq!(v.clone(), v);
}

#[test]
fn default() {
    let v = HeaderVec::<bool, i32>::default();
    assert_eq!(v, header_vec![false;]);
}
