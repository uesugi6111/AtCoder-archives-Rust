#[allow(unused_imports)]
use proconio::{fastout, input};
use std::string::String;
#[fastout]
fn main() {
    input!(n: usize, s: String);

    let (ff, ss) = s.split_at(n / 2);

    println!("{}", if ff == ss { "Yes" } else { "No" });
}
