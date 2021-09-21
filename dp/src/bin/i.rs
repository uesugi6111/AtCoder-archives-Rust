#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
fn main() {
    input!(n: usize, p: [f64; n]);

    let mut dp = vec![vec![0.0; n + 1]; n + 1];
    dp[0][0] = 1.0;
    for i in 1..n + 1 {
        for j in 0..i + 1 {
            if j > 0 {
                dp[i][j] = dp[i - 1][j - 1] * p[i - 1];
            }
            dp[i][j] += dp[i - 1][j] * (1.0 - p[i - 1]);
        }
    }
    let ans: f64 = dp[n].iter().skip((n + 1) / 2).sum();
    println!("{}", ans);
}
