#[allow(unused_imports)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(m: usize, n: usize, nn: usize);

    let mut count = nn;
    let mut hanbai = nn;

    while hanbai >= m {
        count += hanbai / m * n;
        hanbai = hanbai / m * n + hanbai % m;
    }

    println!("{}", count);
}
