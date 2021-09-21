#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
fn main() {
    input!(n: usize, p: [usize; n]);

    let mut pp = vec![0; 3];
    p.iter().for_each(|x| pp[*x - 1] += 1);

    let mut memo = vec![vec![vec![-1.0; n + 1]; n + 1]; n + 1];
    let mut flg = vec![vec![vec![false; n + 1]; n + 1]; n + 1];
    let ans = excute(n, pp[0], pp[1], pp[2], &mut memo, &mut flg);

    println!("{}", ans);
}

fn excute(
    n: usize,
    a1: i64,
    a2: i64,
    a3: i64,
    memo: &mut Vec<Vec<Vec<f64>>>,
    flg: &mut Vec<Vec<Vec<bool>>>,
) -> f64 {
    if a1 == 0 && a2 == 0 && a3 == 0 {
        return 0.0;
    }

    if flg[a1 as usize][a2 as usize][a3 as usize] {
        return memo[a1 as usize][a2 as usize][a3 as usize];
    }
    flg[a1 as usize][a2 as usize][a3 as usize] = true;

    let per = 1.0 - (n as i64 - a1 - a2 - a3) as f64 / n as f64;
    let mut ans = 1.0 / per;

    if a1 != 0 {
        ans += excute(n, a1 - 1, a2, a3, memo, flg) * (a1 as f64) / (n as f64) / per;
    }

    if a2 != 0 {
        ans += excute(n, a1 + 1, a2 - 1, a3, memo, flg) * (a2 as f64) / (n as f64) / per;
    }

    if a3 != 0 {
        ans += excute(n, a1, a2 + 1, a3 - 1, memo, flg) * (a3 as f64) / (n as f64) / per;
    }
    memo[a1 as usize][a2 as usize][a3 as usize] = ans;
    memo[a1 as usize][a2 as usize][a3 as usize]
}
