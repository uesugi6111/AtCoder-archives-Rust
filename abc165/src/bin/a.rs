#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
fn main() {
    input!(k: usize, a: usize, b: usize);

    println!("{}", if (b / k) * k >= a { "OK" } else { "NG" });
}
