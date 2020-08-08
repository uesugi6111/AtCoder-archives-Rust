#[allow(unused_imports)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(a: isize, b: isize, c: isize);
    println!("{}", if 0 < c - (a - b) { c - (a - b) } else { 0 });
}
