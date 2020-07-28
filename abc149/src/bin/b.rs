#[allow(unused_imports)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(mut a: usize, mut b:usize,k:usize);
    if a < k {
        if b < (k - a) {
            b = 0;
        } else {
            b -= k - a;
        }
        a = 0;
    } else {
        a -= k;
    }
    println!("{} {}", a, b);
}
