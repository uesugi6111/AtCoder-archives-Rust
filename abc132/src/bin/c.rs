#[allow(unused_imports)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(n: usize, mut d: [usize; n]);

    d.sort();

    println!("{}", d[n / 2] - d[n / 2 - 1]);
}
