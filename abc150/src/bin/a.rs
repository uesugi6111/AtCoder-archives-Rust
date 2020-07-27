#[allow(unused_imports)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(k: usize, x: usize);
    println!("{}", if 500 * k >= x { "Yes" } else { "No" });
}
