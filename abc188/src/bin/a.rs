#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
fn main() {
    input!(x: i64, y: i64);

    println!("{}", if (x - y).abs() < 3 { "Yes" } else { "No" });
}
