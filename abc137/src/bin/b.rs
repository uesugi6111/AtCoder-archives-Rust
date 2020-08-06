#[allow(unused_imports)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(k: isize, x: isize);
    for i in x - k + 1..k + x {
        print!("{} ", i);
    }
}
