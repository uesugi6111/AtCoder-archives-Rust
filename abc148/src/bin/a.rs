#[allow(unused_imports)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(a: usize, b: usize);
    let c = if a == 1 {
        if b == 2 {
            3
        } else {
            2
        }
    } else if a == 2 {
        if b == 1 {
            3
        } else {
            1
        }
    } else {
        if b == 1 {
            2
        } else {
            1
        }
    };
    println!("{}", c);
}
