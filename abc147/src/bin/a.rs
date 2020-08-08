#[allow(unused_imports)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(a_1: usize, a_2: usize, a_3: usize);
    println!("{}", if a_1 + a_2 + a_3 >= 22 { "bust" } else { "win" });
}
