#[allow(unused_imports)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(r: usize, d: usize, x: usize);

    let mut buff = x;
    for _i in 0..10 {
        println!("{}", {
            buff = buff * r - d;
            buff
        });
    }
}
