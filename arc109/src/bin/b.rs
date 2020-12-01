#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
fn main() {
    input!(n: i64);
    let ans = sequential_search(n + 1);
    println!("{}", n + 1 - ans);
}

fn binary_search(n: i64) -> i64 {
    let mut left = -1;
    let mut right = n;

    while right - left > 1 {
        let mid = left + (right - left) / 2;

        if is_ok(mid, n) {
            right = mid;
        } else {
            left = mid
        };
    }

    left
}

fn is_ok(index: i64, n: i64) -> bool {
    index as u128 * (index as u128 + 1) / 2 > n as u128
}

fn sequential_search(n: i64) -> i64 {
    if n == 2 {
        return 1;
    }
    if n == 3 {
        return 2;
    }

    let nn = n as u128 * 2;

    for i in 0..n {
        if i as u128 * (i as u128 + 1) > nn {
            return (i - 1) as i64;
        }
    }

    n
}
