#[allow(unused_imports)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(a: u64,mut b: u64, mut c: u64, mut k: u64);

    let mut count: u64 = 0;
    while a >= b {
        b *= 2;
        count += 1;
    }
    while b >= c {
        c *= 2;
        count += 1;
    }

    println!("{}", if count > k { "No" } else { "Yes" });
}
