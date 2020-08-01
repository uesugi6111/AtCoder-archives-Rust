#[allow(unused_imports)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(n: usize, k: usize, h: [usize; n]);
    let mut count = 0;
    for v in h {
        if v >= k {
            count += 1;
        }
    }

    println!("{}", count);
}
