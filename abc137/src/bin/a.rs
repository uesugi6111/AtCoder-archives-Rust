#[allow(unused_imports)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(a: isize, b: isize);
    let ab = vec![a + b, a - b, a * b];

    println!("{}", ab.iter().max().unwrap());
}
