#[allow(unused_imports)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(x: usize, a: usize);

    println!("{}", if x < a { 0 } else { 10 });
}
