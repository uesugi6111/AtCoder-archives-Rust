#[allow(unused_imports)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(_n: usize, s: String);

    println!("{}", s.match_indices("ABC").count());
}
