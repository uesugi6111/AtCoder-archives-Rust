#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
fn main() {
    input!(h: usize, w: usize, a: [usize; w * h]);

    let min = a.iter().min().unwrap();

    let ans: usize = a.iter().map(|x| x - min).sum();

    println!("{}", ans);
}
