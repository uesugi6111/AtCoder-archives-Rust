#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
fn main() {
    input!(n: usize, k: usize, h: [i64; n]);

    let mut dp = vec![std::i64::MAX; n];
    dp[0] = 0;

    for i in 0..n {
        for j in 1..=std::cmp::min(n - i - 1, k) {
            dp[i + j] = std::cmp::min(dp[i + j], dp[i] + (h[i] - h[i + j]).abs());
        }
    }

    println!("{}", dp[n - 1]);
}
