#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
fn main() {
    input!(n: usize, x: [[isize; 3]; n]);

    println!("{}", held_karp(n, 0, true, x));
}

fn held_karp(n: usize, start: usize, restart: bool, x: Vec<Vec<isize>>) -> usize {
    let mut dp: Vec<Vec<usize>> = vec![vec![std::usize::MAX; n]; (1 << n) as usize];
    dp[1 << start][start] = 0;

    for i in 0..1 << n {
        for from in 0..n {
            if dp[i][from] == std::usize::MAX {
                continue;
            }

            for to in 0..n {
                if (i & 1 << to) != 0 {
                    continue;
                }
                let next = i | 1 << to;
                let dist = dp[i][from] + g(&x, from, to, i);
                if dist < dp[next][to] {
                    dp[next][to] = dist;
                }
            }
        }
    }

    let mut ans = std::usize::MAX;
    for i in 0..n {
        ans = std::cmp::min(
            ans,
            dp[(1 << n) - 1][i]
                + (if restart {
                    g(&x, i, start, (1 << n) - 1)
                } else {
                    0
                }),
        );
    }
    ans
}
fn g(x: &[Vec<isize>], from: usize, to: usize, _i: usize) -> usize {
    ((x[to][0] - x[from][0]).abs()
        + (x[to][1] - x[from][1]).abs()
        + std::cmp::max(0, x[to][2] - x[from][2])) as usize
}
