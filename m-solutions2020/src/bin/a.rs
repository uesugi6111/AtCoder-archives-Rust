#[allow(unused_imports)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(x: u64);
    println!("{}", 8 - (x - 400) / 200);
}
