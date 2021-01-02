#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
fn main() {
    input!(n: f64);
    println!("{}", (9.0 / 5.0 * n) + 32.0);
}
