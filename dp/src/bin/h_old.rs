use std::vec;

#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
fn main() {
    input!(h: usize, w: usize, a: [Chars; h]);

    let mut memo = vec![vec![-1; w]; h];

    println!("{}", excute(w - 1, h - 1, &a, &mut memo));
}

fn excute(x: usize, y: usize, a: &[Vec<char>], memo: &mut Vec<Vec<i64>>) -> i64 {
    if memo[y][x] != -1 {
        return memo[y][x];
    }
    let mut ret = 0;

    if x != 0 && a[y][x - 1] == '.' {
        ret += excute(x - 1, y, a, memo);
    }

    if y != 0 && a[y - 1][x] == '.' {
        ret += excute(x, y - 1, a, memo);
    }

    if x == 0 && y == 0 {
        ret += 1;
    }
    memo[y][x] = ret % 1_000_000_007;
    memo[y][x]
}
