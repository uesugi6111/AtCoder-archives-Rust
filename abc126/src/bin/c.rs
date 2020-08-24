#[allow(unused_imports)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(n: usize, k: usize);

    let mut kakuritsu: f64 = 0.0;
    for i in 1..n + 1 {
        let mut buff = i;
        let mut count = 0;
        loop {
            if buff >= k {
                break;
            }
            buff *= 2;
            count += 1;
        }

        kakuritsu += 1.0 / (1 << count) as f64;
    }

    println!("{}", kakuritsu / n as f64);
}
