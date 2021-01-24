#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
fn main() {
    input!(n: usize, x: i64, vp: [(i64, i64); n]);

    let mut a = 0;
    for (i, v) in vp.iter().enumerate() {
        a += v.0 * v.1;
        if a > x * 100 {
            println!("{}", i + 1);
            return;
        }
    }

    println!("-1");
}
