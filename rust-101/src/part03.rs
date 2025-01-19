/*
* Replaces read_vec by a function which accepts user input.
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

pub fn vec_min<T: Minimum>(v: Vec<T>) -> SomethingOrNothing<T> {
    let mut min = Nothing;
    for e in v {
        min = Something(match min {
            Nothing => e,
            Something(n) => {
                e.min(n)
            }
        });
    }
    min
}

impl Minimum for i32 {
    fn min(self, b: Self) -> Self {
        if self < b { self } else { b }
    }
}

impl Minimum for f32 {
    fn min(self, b: Self) -> Self {
        if self < b { self } else { b }
    }
}

impl<T: std::fmt::Display> NumOrNothing<T> {
    fn print(self) {
        match self {
            Nothing => println!("\nThe number is <nothing>"),
            Something(n) => println!("\nThe min number is: {:.2}", n),
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

pub fn main() {
    // let vec = read_vec();
    let vec: Vec::<f32> = read_vec::<f32>();
    let min = vec_min(vec.clone());
    let min_clone = vec_min(vec);
    min.print();
    min_clone.print2();
}

pub trait Print {
    fn println_t<T: std::fmt::Display>(self, v: T);
}

impl Print for i32 {
    fn println_t<I32: std::fmt::Display>(self, v: I32) {
        println!("{:.2}", v);
    }
}

impl Print for f32 {
    fn println_t<F32: std::fmt::Display>(self, v: F32) {
        println!("{:.2}", v);
    }
}

impl<T: Print + std::fmt::Display>SomethingOrNothing<T> {
    fn print2(self) {
        match self {
            Nothing => println!("The number is: <nothing>"),
            Something(t) => println!("The min number is: {:.2}", t),
        };
    }
}
