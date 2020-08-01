#[allow(unused_imports)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(a: usize, b: usize);
    if a > 9 || b > 9 {
        println!("{}", -1);
        return;
    }

    println!("{}", a * b);
}
