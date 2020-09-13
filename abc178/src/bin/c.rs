#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
const MMMOD: isize = 1_000_000_007;
#[fastout]
fn main() {
    input!(n: isize);

    let ans =
        powmod(10, n) + powmod(9, n).wrapping_neg() + powmod(9, n).wrapping_neg() + powmod(8, n);

    println!("{}", (ans % MMMOD + MMMOD) % MMMOD);
}

fn powmod(x: isize, y: isize) -> isize {
    let mut res = 1;
    for _i in 0..y {
        res = res * x % MMMOD;
    }
    res
}
