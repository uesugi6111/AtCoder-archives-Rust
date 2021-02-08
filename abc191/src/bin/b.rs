#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
fn main() {
    input!(n: usize, x: i64, a: [i64; n]);

    for v in a {
        if v != x {
            print!("{} ", v);
        }
    }
}
