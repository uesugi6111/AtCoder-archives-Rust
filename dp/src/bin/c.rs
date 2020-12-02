use std::vec;

#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
fn main() {
    input!(n: usize, abc: [[i64; 3]; n]);
    let mut dp = vec![vec![0i64; 3]; n];
    for (i, v) in abc[0].iter().enumerate() {
        dp[0][i] = *v;
    }

    for (index, v) in abc.iter().enumerate().skip(1) {
        for (i, j) in v.iter().enumerate() {
            dp[index][i] = dp[index - 1]
                .iter()
                .enumerate()
                .filter(|x| x.0 != i)
                .map(|x| x.1 + j)
                .max()
                .unwrap();
        }
    }
    println!("{}", dp[n - 1].iter().max().unwrap());
}
