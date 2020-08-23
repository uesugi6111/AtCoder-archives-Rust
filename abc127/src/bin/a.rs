#[allow(unused_imports)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(a: usize, b: usize);

    println!(
        "{}",
        if a >= 13 {
            b
        } else if a >= 6 {
            b / 2
        } else {
            0
        }
    );
}
