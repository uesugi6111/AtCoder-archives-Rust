#[allow(unused_imports)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(n: usize);
    let mut count = 0;
    for i in 0..n {
        if i % 2 == 0 {
            count += 1;
        }
    }

    println!("{}", count as f64 / n as f64);
}
