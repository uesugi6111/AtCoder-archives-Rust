#[allow(unused_imports)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(a: usize, b: usize, t: usize);

    println!("{}", (((t as f64 + 0.5) / a as f64) as usize * b));
}
