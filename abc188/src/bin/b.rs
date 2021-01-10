#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
fn main() {
    input!(n: usize, a: [i64; n], b: [i64; n]);

    let mut sum = 0;
    for i in 0..n {
        sum += a[i] * b[i];
    }
    println!("{}", if sum == 0 { "Yes" } else { "No" });
}
