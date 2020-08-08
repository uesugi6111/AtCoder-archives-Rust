#[allow(unused_imports)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(n: usize, a: usize, b: usize);
    let ans = if a * n < b { a * n } else { b };
    println!("{}", ans);
}
