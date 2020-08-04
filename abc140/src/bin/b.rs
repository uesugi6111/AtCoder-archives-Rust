#[allow(unused_imports)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(n: usize, a: [usize; n], b: [usize; n], c: [usize; n - 1]);

    let mut score = 0;
    let mut pre_b = a[0];
    for v in a {
        score += b[v - 1];
        if pre_b + 1 == v - 1 {
            score += c[pre_b];
        }
        pre_b = v - 1;
    }

    println!("{}", score);
}
