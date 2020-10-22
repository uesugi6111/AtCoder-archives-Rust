#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
fn main() {
    input!(a: u128, b: f64);

    let bb = ((b * 100.0) + 0.5) as u128;

    println!("{}", (a * bb) / 100);
}
