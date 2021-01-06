#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
fn main() {
    input!(
        n: usize,
        ab: [(i64, i64); n - 1],
        q: usize,
        tex: [(i64, i64, i64); q]
    );

    println!("{}", n);
}
