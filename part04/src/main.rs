/*
* Some examples of shared references.
*/
use std::io::prelude::*;
use std::io;
pub enum SomethingOrNothing<T> {
    Something(T),
    Nothing
}

pub trait Minimum : Copy {
    fn min(self, b: Self) -> Self;
}

pub use self::SomethingOrNothing::*;
type NumOrNothing<T> = SomethingOrNothing<T>;

impl<T> SomethingOrNothing<T> {
    fn new(o: Option<T>) -> Self {
        match o { None => Nothing, Some(t) => Something(t) }
    }

    fn to_option(self) -> Option<T> {
        match self { Nothing => None, Something(t) => Some(t) }
    }
}

pub fn vec_min<T: std::cmp::Ord>(v: &Vec<i32>) -> Option<i32> {
    use std::cmp;

    let mut min = None;
    // borrowing the vector, providing shared references
    for e in v.iter() {
        min = Some(match min {
            None => *e,
            Some(n) => cmp::min(n, *e)
        });
    }
    min
}

impl Minimum for i32 {
    fn min(self, b: Self) -> Self {
        if self < b { self } else { b }
    }
}

impl<T: std::fmt::Display> NumOrNothing<T> {
    fn print(self) {
        match self {
            Nothing => println!("\nThe number is <nothing>"),
            Something(n) => println!("\nThe min number is: {}", n),
        };
    }
}

fn read_vec<T: std::str::FromStr>() -> Vec<T> {
    // a new empty vector
    let mut vec: Vec<T> = Vec::new();
    let stdin = io::stdin();
    println!("Please enter a list of numbers, one per line. End with Ctrl-D (Linux) or Ctrl-Z (windows).");

    for line in stdin.lock().lines() {
        let line = line.unwrap();
        match line.trim().parse::<T>() {
            Ok(num) => {
                vec.push(num)
            },
            Err(_) => {
                println!("Please enter a number")
            },
        }
    }
    vec
}

fn vec_inc(v: &mut Vec<i32>) -> &mut Vec<i32> {
    for e in v.iter_mut() {
        *e += 1;
    }
    v
}

pub fn main() {
    // let vec = read_vec();
    let v = vec![5,4,3,2,1];
    let first = &v[0];
    vec_min::<i32>(&v);
    vec_min::<i32>(&v);
    println!("The first element is: {}", *first);

    let mut v = vec![6,5,4,3,2,1];
    vec_inc(&mut v);
    vec_inc(&mut v);
    println!("{:?}", v);
}

