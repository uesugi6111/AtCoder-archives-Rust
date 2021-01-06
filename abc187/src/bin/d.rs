use itertools_num::*;
#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
use std::cmp::Reverse;
#[fastout]
fn main() {
    input!(n: usize, ab: [(i64, i64); n]);

    let aoki: i64 = ab.iter().map(|x| x.0).sum();

    let mut c: Vec<i64> = ab.iter().map(|x| x.0 * 2 + x.1).collect();

    c.sort_by_key(|&x| Reverse(x));

    for i in c.iter().cumsum::<i64>().enumerate() {
        if i.1 > aoki {
            println!("{}", i.0 + 1);
            return;
        }
    }

    println!("{}", n);
}
