#[allow(unused_imports)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(n: usize, m: usize, lr: [[usize; 2]; m]);

    let mut result_vec = vec![0 as isize; n + 1];
    for v in lr {
        result_vec[v[0] - 1] += 1;
        result_vec[v[1]] -= 1;
    }

    let mut ruiseki = Vec::new();
    ruiseki.push(result_vec[0]);
    for i in 1..result_vec.len() {
        ruiseki.push(ruiseki[i - 1] + result_vec[i]);
    }

    println!("{}", ruiseki.iter().filter(|r| **r == m as isize).count());
}
