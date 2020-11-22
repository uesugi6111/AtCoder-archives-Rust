#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
fn main() {
    input!(a: (i64, i64, i64, i64));

    println!("{}", a.0 * a.3 - a.1 * a.2);
}
