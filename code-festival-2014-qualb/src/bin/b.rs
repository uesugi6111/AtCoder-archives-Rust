use itertools_num::*;
#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
fn main() {
    input!(n: usize, k: i64, a: [i64; n]);

    for v in a.iter().cumsum().enumerate() {
        if k <= v.1 {
            println!("{}", v.0 + 1);
            return;
        }
    }
}
