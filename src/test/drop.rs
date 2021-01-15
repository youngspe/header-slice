use crate::header_vec;
use alloc::vec::Vec;
use core::cell::RefCell;

macro_rules! qdrop {
    (struct $name:ident(..);) => {
        #[derive(Clone, Debug)]
        struct $name<'a, T: Clone>(T, &'a RefCell<Vec<T>>);

        impl<'a, T: Clone> Drop for $name<'a, T> {
            fn drop(&mut self) {
                self.1.borrow_mut().push(self.0.clone());
            }
        }
    };
}

#[test]
fn drop_items() {
    qdrop! { struct A(..); }
    let q = RefCell::new(Vec::new());
    let mut v = header_vec!["foo"; A(1, &q), A(2, &q)];
    let popped = v.pop().unwrap();
    v.push(A(3, &q));
    core::mem::drop(v);
    core::mem::drop(popped);
    assert_eq!(**q.borrow(), [1, 3, 2]);
}

#[test]
fn drop_head() {
    qdrop! { struct A(..); }
    let q = RefCell::new(Vec::new());
    let v = header_vec![A("foo", &q); 1, 2, 3];
    core::mem::drop(v);
    assert_eq!(**q.borrow(), ["foo"]);
}

#[test]
fn values_iter_drops() {
    qdrop! { struct A(..); }
    qdrop! { struct B(..); }
    // queue for dropped items
    let q = RefCell::new(Vec::new());
    // queue for dropped header
    let q_h = RefCell::new(Vec::<&str>::new());

    let mut v = header_vec![A("head", &q_h); B(1, &q), B(2, &q), B(3, &q)];
    v.push(B(4, &q));
    assert!(q.borrow().is_empty());
    assert!(q_h.borrow().is_empty());

    let mut vals = v.into_values();
    assert!(q.borrow().is_empty());
    assert_eq!(**q_h.borrow(), ["head"]);

    core::mem::drop(vals.next().unwrap());
    assert_eq!(**q.borrow(), [1]);

    core::mem::drop(vals.next().unwrap());
    assert_eq!(**q.borrow(), [1, 2]);

    core::mem::drop(vals);
    assert_eq!(**q.borrow(), [1, 2, 3, 4]);
    assert_eq!(**q_h.borrow(), ["head"]);
}

#[test]
fn drop_items_on_clear() {
    qdrop! { struct A(..); }
    let q = RefCell::new(Vec::new());
    let mut v = header_vec!["foo"; A(1, &q), A(2, &q), A(3, &q)];
    v.clear();
    assert_eq!(**q.borrow(), [1, 2, 3]);
}

#[test]
fn drop_items_on_clear_in_place() {
    qdrop! { struct A(..); }
    let q = RefCell::new(Vec::new());
    let mut v = header_vec!["foo"; A(1, &q), A(2, &q), A(3, &q)];
    let old_cap = v.capacity();
    v.clear_in_place();
    assert_eq!(**q.borrow(), [1, 2, 3]);
    assert_eq!(v.capacity(), old_cap);
}

#[test]
fn drop_items_on_truncate() {
    qdrop! { struct A(..); }
    let q = RefCell::new(Vec::new());
    let mut v = header_vec!["foo"; A(1, &q), A(2, &q), A(3, &q)];
    v.truncate(1);
    assert_eq!(**q.borrow(), [2, 3]);
}

#[test]
fn drop_items_on_resize() {
    qdrop! { struct A(..); }
    let q = RefCell::new(Vec::new());
    let mut v = header_vec!["foo"; A(1, &q), A(2, &q), A(3, &q)];
    v.resize(1, A(7, &q));
    // sort this so we don't depend on when 7 gets dropped.
    (**q.borrow_mut()).sort();
    assert_eq!(**q.borrow(), [2, 3, 7]);
}

#[test]
fn drop_items_on_resize_with() {
    qdrop! { struct A(..); }
    let q = RefCell::new(Vec::new());
    let mut v = header_vec!["foo"; A(1, &q), A(2, &q), A(3, &q)];
    v.resize_with(1, || panic!());
    assert_eq!(**q.borrow(), [2, 3]);
}

#[test]
fn drop_items_on_resize_with_default() {
    qdrop! { struct A(..); }
    impl<'a, T: Clone> Default for A<'a, T> {
        fn default() -> Self {
            panic!();
        }
    }
    let q = RefCell::new(Vec::new());
    let mut v = header_vec!["foo"; A(1, &q), A(2, &q), A(3, &q)];
    v.resize_default(1);
    assert_eq!(**q.borrow(), [2, 3]);
}
