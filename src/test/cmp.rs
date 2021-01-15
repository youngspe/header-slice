use crate::{header_vec, HeaderVec};
use core::ops::Deref;

#[test]
fn eq() {
    let foo_empty: HeaderVec<_, i32> = header_vec!["foo";];
    assert_eq!(header_vec!["foo"; 1, 2, 3], header_vec!["foo"; 1, 2, 3]);
    assert_eq!(foo_empty, foo_empty);
    assert_eq!(header_vec!["foo"; 1, 2], *header_vec!["foo"; 1, 2].deref());
    assert_eq!(*header_vec!["foo"; 1, 2].deref(), header_vec!["foo"; 1, 2]);
    assert_eq!(
        header_vec!["foo"; 1, 2].deref(),
        header_vec!["foo"; 1, 2].deref()
    );
}

#[test]
fn ne() {
    let foo_empty: HeaderVec<_, i32> = header_vec!["foo";];
    let bar_empty: HeaderVec<_, i32> = header_vec!["bar";];
    assert_ne!(header_vec!["foo"; 1, 2, 3], header_vec!["foo"; 2, 2, 3]);
    assert_ne!(header_vec!["foo"; 1, 2], header_vec!["foo"; 1, 2, 3]);
    assert_ne!(foo_empty, header_vec!["foo"; 1]);
    assert_ne!(header_vec!["foo"; 1, 2], header_vec!["foo";]);
    assert_ne!(
        header_vec!["foo"; 1, 2, 3],
        *header_vec!["foo"; 2, 3].deref()
    );
    assert_ne!(
        *header_vec!["foo"; 1, 2, 3].deref(),
        header_vec!["foo"; 2, 3]
    );
    assert_ne!(
        header_vec!["foo"; 1, 2, 3].deref(),
        header_vec!["foo"; 2, 3].deref()
    );

    assert_ne!(header_vec!["foo"; 1, 2, 3], header_vec!["bar"; 1, 2, 3]);
    assert_ne!(foo_empty, bar_empty);
}

macro_rules! assert_ord {
    ($a:expr, $b:expr, $cmp:ident) => {
        assert_eq!($a.cmp(&$b), core::cmp::Ordering::$cmp);
        assert_ord!(?$a, $b, $cmp);
    };
    (? $a:expr, $b:expr, $cmp:ident) => {
        assert_eq!($a.partial_cmp(&$b), Some(core::cmp::Ordering::$cmp));
    };
    ({ $a:expr } ?< { $b:expr }) => {
        assert_ord!(?$a, $b, Less);
        assert_ord!(?$b, $a, Greater);
    };
    ({ $a:expr } < { $b:expr }) => {
        assert_ord!($a, $b, Less);
        assert_ord!($b, $a, Greater);
    };
    ({ $a:expr } ?== { $b:expr }) => {
        assert_ord!(?$a, $b, Equal);
        assert_ord!(?$b, $a, Equal);
    };
    ({ $a:expr } == { $b:expr }) => {
        assert_ord!($a, $b, Equal);
        assert_ord!($b, $a, Equal);
    };
    ({ $a:expr } ?> { $b:expr }) => {
        assert_ord!(?$a, $b, Greater);
        assert_ord!(?$b, $a, Less);
    };
    ({ $a:expr } > { $b:expr }) => {
        assert_ord!($a, $b, Greater);
        assert_ord!($b, $a, Less);
    };
    ({ $a:expr } ? { $b:expr }) => {
        assert_eq!($a.partial_cmp(&$b), None);
        assert_eq!($b.partial_cmp(&$a), None);
    };
}

#[test]
fn ord() {
    let a_empty: HeaderVec<_, i32> = header_vec!["a";];
    let b_empty: HeaderVec<_, i32> = header_vec!["b";];
    assert_ord!({ a_empty } == { a_empty });
    assert_ord!({ a_empty } < { b_empty });

    assert_ord!({ header_vec!["a"; 1, 2] } == { header_vec!["a"; 1, 2] });
    assert_ord!({ header_vec!["a"; 1, 2] } < { header_vec!["b"; 1, 2] });
    assert_ord!({ header_vec!["a"; 1, 2] } < { header_vec!["b"; 1, 2, 3] });
    assert_ord!({ header_vec!["a"; 1, 2] } < { header_vec!["a"; 1, 3] });
    assert_ord!({ header_vec!["a"; 1, 2] } < { header_vec!["a"; 1, 2, 3] });

    assert_ord!({ header_vec![1.0; 1, 2] }? == { header_vec![1.0; 1, 2] });
    assert_ord!({ header_vec![1.0; 1, 2] }? < { header_vec![1.0; 1, 3] });
    assert_ord!({ header_vec![1.0; 1, 2] }? < { header_vec![2.0; 1, 2] });

    assert_ord!({ header_vec![f32::NAN; 1, 2] } ? { header_vec![f32::NAN; 1, 2] });
    assert_ord!({ header_vec![1.0; 1, 2] } ? { header_vec![f32::NAN; 1, 2] });
    assert_ord!({ header_vec!["foo"; 1.0, 2.0] }? == { header_vec!["foo"; 1.0, 2.0] });
    assert_ord!({ header_vec!["foo"; 1.0, 2.0] }? < { header_vec!["foo"; 1.0, 3.0] });
    assert_ord!({ header_vec!["foo"; 1.0, 2.0] }? < { header_vec!["foo"; 1.0, 2.0, 3.0] });
}
