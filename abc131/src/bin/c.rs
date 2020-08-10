#[allow(unused_imports)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(a: usize, b: usize, c: usize, d: usize);

    let ab: u128 = (b - (a - 1)) as u128;

    let cd = yukuriddo(c, d) as u128;

    let abcd = b as u128 / cd - (a - 1) as u128 / cd;

    let ans = (b / c - (a - 1) / c) as u128 + (b / d - (a - 1) / d) as u128 - abcd;

    println!("{}", ab - ans);
}

fn yukuriddo(a_: usize, b_: usize) -> usize {
    let mut a = a_;
    let mut b = b_;
    let x = a * b;
    let tmp;

    /* 自然数 a > b を確認・入替 */
    if a < b {
        tmp = a;
        a = b;
        b = tmp;
    }
    /* ユークリッドの互除法 */
    let mut r = a % b;
    while r != 0 {
        a = b;
        b = r;
        r = a % b;
    }

    return x / b;
}
