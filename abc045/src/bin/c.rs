#[allow(unused_imports)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(_y: usize);
    println!("{}", Ok(1).unwrap_or_else(|e| e));
}
