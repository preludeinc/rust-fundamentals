/* 
* Building a data structure for computation with big numbers.
*
*/
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

    pub fn from_vec(mut v: Vec<u64>) -> Self {
        let val = v.iter().fold(0, |acc, elem| acc * 10 + elem);
        let mut mod_vec = vec![val];
        while let Some(last) = mod_vec.last() {
            if *last == 0 {
                mod_vec.pop();
            } else {
                break;
            }
        }
        BigInt { data: mod_vec }
    }
}

fn clone_demo() {
    let v = vec![0,1 << 16];
    let b1 = BigInt::from_vec((&v).clone());
    let b2 = BigInt::from_vec(v);
    println!("{:?}", b1);
    println!("{:?}", b2);
}

pub fn main() {
    clone_demo();
}