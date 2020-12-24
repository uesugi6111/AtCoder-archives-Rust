#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
fn main() {
    input!(l: i64);

    println!("{}", comb(l - 1, 11));
}

fn comb(n: i64, r: i64) -> i64 {
    match (n, r) {
        (0, _) | (_, 0) => 1,
        _ => comb(n, r - 1) * (n - r + 1) / r,
    }
}
