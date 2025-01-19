/*
* Continuation of BigInt class.
*
*/
mod part05;
use part05::BigInt;

impl BigInt {
    fn min_try1(self, other: Self) -> Self {
        debug_assert!(self.test_invariant() && other.test_invariant());
    
        if self.data.len() < other.data.len() {
            self
        } else if self.data.len() > other.data.len() {
            other
        } else {
            if self.data > other.data {
                self
            }
            else {
                other
            }
        }
    }
}

fn vec_min(v: &Vec<BigInt>) ->Option<BigInt> {
    let mut min: Option<BigInt> = None;

    for e in v {
        let e = e.clone();
        min = Some(match min {
            None => e,
            Some(n) => e.min_try1(n)
        });
    }
    min
}

use part05::SomethingOrNothing;
impl<T: Copy> Copy for SomethingOrNothing<T>{}

fn head<T>(v: &Vec<T>) -> Option<&T> {
    if v.len() > 0 {
        Some(&v[0])
    } else {
        None
    }
}

fn rust_foo(mut v: Vec<i32>) -> i32 {
    let first: Option<&i32> = head(&v);
    /* v.push(42); */
    *first.unwrap()
}

fn main() {
    let v1 = BigInt::new(6 << 16);
    let v2 = BigInt::new(1 << 16);
    println!("{:?}", v1);
    println!("{:?}", v2);
    let b1 = BigInt::min_try1(v1, v2);
    println!("{:?}", b1);
}
