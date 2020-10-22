#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
fn main() {
    input!(n: usize, x: [isize; n]);

    println!("{}", x.iter().map(|x| x.abs()).sum::<isize>());

    println!("{}", x.iter().map(|x| (x * x) as f64).sum::<f64>().sqrt());

    println!("{}", x.iter().map(|x| x.abs()).max().unwrap());
}
