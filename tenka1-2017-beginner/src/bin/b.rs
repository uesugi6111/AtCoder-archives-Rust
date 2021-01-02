#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
fn main() {
    input!(n: usize,mut ab:[(i64,i64);n]);

    ab.sort_by_key(|x| x.0);

    println!("{}", ab[ab.len() - 1].0 + ab[ab.len() - 1].1);
}
