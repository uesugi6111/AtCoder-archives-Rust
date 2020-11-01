#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
fn main() {
    input!(n: usize, ab: [(usize, usize); n]);

    let mut sum = 0;
    for (a, b) in ab {
        sum += (a + b) * (b - a + 1) / 2;
    }

    println!("{}", sum);
}
