#[allow(unused_imports)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(n: i64, m: i64);
    println!("{}", if n == m { "Yes" } else { "No" });
}
