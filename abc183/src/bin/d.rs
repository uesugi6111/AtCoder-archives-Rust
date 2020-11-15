#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
fn main() {
    input!(n: usize, w: isize, stp: [(usize, usize, usize); n]);

    let mut imos = vec![0isize; 200_002];

    for i in stp {
        imos[i.0 + 1] += i.2 as isize;
        imos[i.1 + 1] += (i.2).wrapping_neg() as isize;
    }

    let mut ruiseki = vec![];
    ruiseki.push(imos[0]);
    for i in 1..imos.len() {
        ruiseki.push(ruiseki[i - 1] + imos[i]);
    }

    let max = ruiseki.iter().max().unwrap();

    println!("{}", if w < *max { "No" } else { "Yes" });
}
