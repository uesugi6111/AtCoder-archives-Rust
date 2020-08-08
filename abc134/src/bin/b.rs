#[allow(unused_imports)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(n: usize, d: usize);
    println!("{}", (n + (2 * d + 1 - 1)) / (2 * d + 1));
}
