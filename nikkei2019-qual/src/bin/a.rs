#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
fn main() {
    input!(n: usize, a: usize, b: usize);

    println!(
        "{} {}",
        std::cmp::min(a, b),
        if n < a + b { a + b - n } else { 0 }
    );
}
