use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(n: usize,k:usize,mut h:[usize;n]);

    h.sort();

    let mut min = 1_000_000_000;
    for i in 0..n - k + 1 {
        let sum: Vec<&usize> = h.iter().skip(i).step_by(k - 1).take(2).collect();
        min = std::cmp::min(min, sum[1] - sum[0]);
    }

    println!("{}", min);
}
