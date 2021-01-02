#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
fn main() {
    input!(n: usize, m: [i64; n]);
    let sum: i64 = m.iter().filter(|x| **x < 80).map(|x| 80 - *x).sum();
    println!("{}", sum);
}
