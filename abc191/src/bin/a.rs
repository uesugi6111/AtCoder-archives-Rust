#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
fn main() {
    input!(v: usize, t: usize, s: usize, d: usize);

    let aaa = d as f64 / v as f64;
    println!(
        "{}",
        if aaa >= t as f64 && s as f64 >= aaa {
            "No"
        } else {
            "Yes"
        }
    );
}
