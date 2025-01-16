/*
* Modifying the syntax from part00; 
* to work with expressions and inherent implementations.
*/
enum NumOrNothing {
    Number(i32),
    Nothing
}

use self::NumOrNothing::{Number,Nothing};
impl NumOrNothing {
    fn number_or_default(self, n: NumOrNothing, default: i32) -> i32 {
        match n {
            Nothing => default,
            Number(n) => n,
        }
    }
}

fn vec_min(v: Vec<i32>) -> NumOrNothing {

    fn min_i32(a: i32, b: i32) -> i32 {
        if a < b { a } else { b }
    }

    let mut min = Nothing;
    for e in v {
        min = Number(match min {
            Nothing => e,
            Number(n) => min_i32(n, e)
        });
    }
    // returns min
    min
}

impl NumOrNothing {
    fn print(self) {
        match self {
            Nothing => println!("The number is <nothing>"),
            Number(n) => println!("The number is: {}", n),
        };
    }
}

fn read_vec() -> Vec<i32> {
    vec![18,5,7,1,9,27]
}

pub fn main() {
    let vec = read_vec();
    let min = vec_min(vec);
    min.print();
}