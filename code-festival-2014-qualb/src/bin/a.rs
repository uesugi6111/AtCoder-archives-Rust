#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
fn main() {
    input!(a: i64, b: i64);
    println!("{}", std::cmp::max(a, b));
}
