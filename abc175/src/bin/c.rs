#[allow(unused_imports)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(mut x: isize, mut k: isize, d: isize);
    x = x.abs();

    let xd = x / d;

    if xd > k {
        println!("{}", (x - (k * d)).abs());
    } else {
        let aaa = x % d;

        let ax = (k - xd) % 2;
        if ax == 0 {
            println!("{}", aaa);
        } else {
            println!("{}", (aaa - d).abs());
        }
    }
}
