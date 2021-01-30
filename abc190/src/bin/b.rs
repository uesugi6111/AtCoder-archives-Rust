#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
fn main() {
    input!(n: usize, s: i64, d: i64, xy: [(i64, i64); n]);

    let count = xy.iter().filter(|x| x.0 < s && x.1 > d).count();

    println!("{}", if count == 0 { "No" } else { "Yes" });
}
