#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
fn main() {
    input!(a: usize, b: usize, x: usize, y: usize);

    let time = if a <= b {
        if 2 * x < y {
            (b - a) * 2 * x + x
        } else {
            (b - a) * y + x
        }
    } else {
        if 2 * x < y {
            (a - b - 1) * 2 * x + x
        } else {
            (a - b - 1) * y + x
        }
    };

    println!("{}", time);
}
