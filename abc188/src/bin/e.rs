#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
fn main() {
    input!(n: usize, m: usize, a: [i64; n], xy: [(i64, i64); m]);

    let mut v = vec![vec![]; n + 1];
    for (x, y) in xy {
        v[x as usize].push(y);
    }

    let mut memo = vec![std::i64::MIN; n + 1];

    let mut max = std::i64::MIN;
    for i in 1..=n {
        if v[i].is_empty() {
            continue;
        }
        max = std::cmp::max(max, execute(&a, &v, i, &mut memo) - a[i - 1]);
    }

    println!("{}", max);
}

fn execute(a: &[i64], v: &[Vec<i64>], x: usize, mut memo: &mut Vec<i64>) -> i64 {
    if memo[x - 1] != std::i64::MIN {
        return memo[x - 1];
    }

    let mut max = std::i64::MIN;
    for i in &v[x] {
        max = std::cmp::max(max, a[*i as usize - 1]);
        max = std::cmp::max(max, execute(&a, &v, *i as usize, &mut memo));
    }

    memo[x - 1] = max;
    max
}
