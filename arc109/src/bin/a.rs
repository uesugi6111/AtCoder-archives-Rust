#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
fn main() {
    input!(a: i64, b: i64, x: i64, y: i64);

    let ab = a - b;

    if ab == 0 {
        println!("{}", x);
        return;
    }

    let ans = if ab < 0 {
        std::cmp::min(ab.abs() * y + x, x + 2 * x * ab.abs())
    } else {
        std::cmp::min((ab.abs() - 1) * y + x, 2 * x * ab.abs() - x)
    };

    println!("{}", ans);
}
