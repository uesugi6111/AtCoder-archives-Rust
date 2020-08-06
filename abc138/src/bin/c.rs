#[allow(unused_imports)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(n: usize, mut y: [usize; n]);
    y.sort();
    let mut buff = y[0] as f64;
    for i in 1..y.len() {
        buff = (buff + y[i] as f64) / 2_f64;
    }
    println!("{}", buff);
}
