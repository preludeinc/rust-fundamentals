/* 
* Building a data structure for computation with big numbers.
*
*/
mod part02;

#[derive(Debug)]
pub struct BigInt {
    pub data: Vec<u64>
}

impl BigInt {
    pub fn new(x: u64) -> Self {
        if x == 0 {
            BigInt { data: vec![] }
        } else {
            BigInt { data: vec![x] }
        }
    }

    pub fn test_invariant(&self) -> bool {
        if self.data.len() == 0 {
            true
        } else {
            self.data[self.data.len() - 1] != 0
        }
    }

    pub fn from_vec(v: Vec<u64>) -> Self {
        let val = v.iter().fold(0, |acc, elem| acc * 10 + elem);

        let mut mod_vec = vec![val];
        while mod_vec.last() == Some(&0) {
            mod_vec.pop();
        }
        
        BigInt { data: mod_vec }
    }
}

impl Clone for BigInt {
    fn clone(&self) -> Self {
        BigInt { data: self.data.clone() }
    }
}

use part02::{SomethingOrNothing, Something, Nothing};
impl<T: Clone> Clone for SomethingOrNothing<T> {
    fn clone(&self) -> Self {
        match *self {
            Nothing => Nothing,
            Something(ref v) => Something(v.clone()),
        }
    }
}

pub fn main() {
    let v = vec![0,1 << 16];
    let b1 = BigInt::from_vec((&v).clone());
    println!("{:?}", b1);
}