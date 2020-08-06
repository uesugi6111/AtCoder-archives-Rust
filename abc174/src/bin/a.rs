#[allow(unused_imports)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(x: isize);

    println!("{}", if x >= 30 { "Yes" } else { "No" });
}
