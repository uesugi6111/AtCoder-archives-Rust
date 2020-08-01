#[allow(unused_imports)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(a: isize, b: isize);
    println!("{}", if a - 2 * b >= 0 { a - 2 * b } else { 0 });
}
