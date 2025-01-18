/*
* Some examples of shared references.
*/
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

