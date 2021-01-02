#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
fn main() {
    input!(r: usize, d: usize);
    println!(
        "{}",
        r as f64 * r as f64 * std::f64::consts::PI * d as f64 * 2.0 * std::f64::consts::PI
    );
}
