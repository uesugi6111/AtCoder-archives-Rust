#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
fn main() {
    input!(sx: f64, sy: f64, gx: f64, gy: f64);

    let x = gx - sx;
    let y = gy + sy;

    println!("{}", sx + x * (sy / y));
}
