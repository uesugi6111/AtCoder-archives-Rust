use std::vec;

#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
fn main() {
    input!(h: usize, w: usize, a: [Chars; h]);

    let mut dp = vec![vec![0; w + 1]; h + 1];
    dp[1][1] = 1;
    for i in 1..h + 1 {
        for j in 1..w + 1 {
            if a[i - 1][j - 1] == '#' || (i == 1 && j == 1) {
                continue;
            }

            dp[i][j] = dp[i][j - 1] + dp[i - 1][j];
            dp[i][j] %= 1_000_000_007;
        }
    }

    println!("{}", dp[h][w]);
}
