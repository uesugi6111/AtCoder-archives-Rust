#[allow(unused_imports)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(n: usize, a: [f64; n]);

    let mut sum = 0.0;
    for v in a {
        sum += 1_f64 / v;
    }
    println!("{}", 1_f64 / sum);
}
