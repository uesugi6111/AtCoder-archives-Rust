#[allow(unused_imports)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(n: usize, k: usize, m: usize, a: [usize; n - 1]);

    let score = a.iter().sum::<usize>();
    let target = n * m;

    println!(
        "{}",
        if target > score + k {
            -1
        } else {
            if target as i64 - score as i64 > 0 {
                target as i64 - score as i64
            } else {
                0
            }
        }
    );
}
