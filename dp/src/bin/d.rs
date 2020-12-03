#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
fn main() {
    input!(n: usize, w: usize, wv: [(usize, i64); n]);

    let mut dp = vec![vec![0i64; w + 1]; n + 1];

    for i in 0..n {
        for j in 0..=w {
            if wv[i].0 <= j {
                dp[i + 1][j] = std::cmp::max(dp[i + 1][j], dp[i][j - wv[i].0] + wv[i].1);
            }
            dp[i + 1][j] = std::cmp::max(dp[i + 1][j], dp[i][j]);
        }
    }

    println!("{}", dp[n][w]);
}
