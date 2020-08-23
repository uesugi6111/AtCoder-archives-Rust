#[allow(unused_imports)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(n: usize, x: usize, t: usize);

    println!("{}", (n + x - 1) / x * t);
}
