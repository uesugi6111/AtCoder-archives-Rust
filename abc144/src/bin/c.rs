#[allow(unused_imports)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(n: usize);

    let n_sqrt = (n as f64).sqrt() as usize;
    let mut min = 1000000000020;
    for i in 1..n_sqrt + 1 {
        if n % i == 0 {
            if min > (i + (n / i) - 2) {
                min = i + n / i - 2;
            }
        }
    }

    println!("{}", min);
}
