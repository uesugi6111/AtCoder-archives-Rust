#[allow(unused_imports)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(a: usize, b: usize);

    let mut aa;
    let mut bb;

    if a < b {
        aa = a;
        bb = b;
    } else {
        aa = b;
        bb = a;
    }

    let mut r = aa % bb;
    while r != 0 {
        aa = bb;
        bb = r;

        r = aa % bb;
    }

    println!("{}", (a * b) / bb);
}
