#[allow(unused_imports)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(n: usize, a: [usize; n]);
    let mut aa = a.clone();
    aa.sort();
    let a_max = aa[n - 1];

    for v in a {
        println!("{}", if a_max == v { aa[n - 2] } else { a_max });
    }
}
