#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
fn main() {
    input!(n: usize, w: i64, wv: [(i64, usize); n]);

    let w_sum_max = 1_000_100;

    let mut dp = vec![vec![1 << 60; w_sum_max + 1]; n + 1];

    dp[0][0] = 0;

    for i in 0..n {
        for j in 0..w_sum_max {
            if j >= wv[i].1 {
                dp[i + 1][j] = std::cmp::min(dp[i + 1][j], dp[i][j - wv[i].1] + wv[i].0);
            }
            dp[i + 1][j] = std::cmp::min(dp[i + 1][j], dp[i][j]);
        }
    }

    let mut ans = 0;

    for (i, j) in dp[n].iter().enumerate() {
        if *j <= w {
            ans = i;
        }
    }
    println!("{}", ans);
}
