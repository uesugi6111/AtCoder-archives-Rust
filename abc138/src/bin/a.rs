#[allow(unused_imports)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(a: usize, s: String);

    println!("{}", if a >= 3200 { s } else { "red".to_string() });
}
