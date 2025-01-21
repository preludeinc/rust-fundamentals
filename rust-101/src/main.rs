/*
* Continuation of BigInt class.
* Featuring operator Overloading.
*
*/
mod part05;
use part05::BigInt;
use std::fmt;

pub trait Minimum {
    fn min<'a>(&'a self, other: &'a Self) -> &'a Self;
}

pub fn vec_min<T: Minimum>(v: &Vec<T>) -> Option<&T> {
    let mut min: Option<&T> = None;
    for e in v {
        min = Some(match min {
            None => e,
            Some(n) => n.min(e)
        });
    }
    min
}

impl Minimum for BigInt {
    fn min<'a>(&'a self, other: &'a Self) -> &'a Self {
        if self.data.len() < other.data.len() {
            self
        } else if self.data.len() > other.data.len() {
            other
        } else if self.data < other.data {
            self
        } else {
            other
        }
    }
}

impl PartialEq for BigInt {
    #[inline]
    fn eq(&self, other: &BigInt) -> bool {
        debug_assert!(self.test_invariant() && other.test_invariant());
        self.data == other.data
    }
}

impl fmt::Debug for BigInt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.data.fmt(f)
    }
}

fn compare_big_ints() {
    let b1 = BigInt::new(13);
    let b2 = BigInt::new(37);
    println!("b1 == b1: {} ; b1 == b2: {}; b1 != b2: {}", b1 == b1, b1 == b2, b1 != b2);
}

#[test]
fn test_min() {
    let b1 = BigInt::new(1);
    let b2 = BigInt::new(42);
    let b3 = BigInt::from_vec(vec![0, 1]);

    assert!(*b1.min(&b2) == b1);
    assert!(*b3.min(&b2) == b2);

    let v1 = vec![b2.clone(), b1.clone(), b3.clone()];
    let v2 = vec![b2.clone(), b3.clone()];

    assert_eq!(vec_min(&v1), Some(&b1));
    assert_eq!(vec_min(&v2), Some(&b2));
}

