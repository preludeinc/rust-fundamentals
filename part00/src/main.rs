/*
* Computes the minimum of a list.
*/
enum NumOrNothing {
    Number(i32),
    Nothing
}

use self::NumOrNothing::{Number,Nothing};

fn vec_min(vec: Vec<i32>) -> NumOrNothing {
    let mut min = NumOrNothing::Nothing;

    for el in vec {
        match min {
            NumOrNothing::Nothing => {
                min = NumOrNothing::Number(el);
            },
            NumOrNothing::Number(n) => {
                let updated_min = min_i32(n, el);
                min = NumOrNothing::Number(updated_min);
            }
        }
    }
    return min;
}

fn min_i32(a: i32, b: i32) -> i32 {
    if a < b {
        return a;
    } else {
        return b;
    }
}

fn read_vec() -> Vec<i32> {
    vec![18,5,7,1,9,27]
}

fn print_number_or_nothing(n:NumOrNothing) {
    match n {
        Nothing => println!("The number is <nothing>"),
        Number(n) => println!("The number is: {}", n),
    };
}

pub fn main() {
    let vec = read_vec();
    let min = vec_min(vec);
    print_number_or_nothing(min);
}