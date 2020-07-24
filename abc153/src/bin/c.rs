#[allow(unused_imports)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(n: usize, k: usize, mut h:  [usize; n]);
    h.sort();
    println!(
        "{}",
        h.iter().take(if n > k { n - k } else { 0 }).sum::<usize>()
    );
}
