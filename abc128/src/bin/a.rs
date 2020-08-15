#[allow(unused_imports)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(a: usize, p: usize);

    println!("{}", (a * 3 + p) / 2);
}
