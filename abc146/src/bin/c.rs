#[allow(unused_imports)]
use proconio::{fastout, input};
#[fastout]
fn main() {
    input!(a: usize, b: usize, x: usize);
    let mut min_n = 0;
    let mut max_n = 1_000_000_000;
    loop {
        if min_n + 1 >= max_n {
            break;
        }
        let target = (min_n + max_n) / 2;
        if a * target + b * (target.to_string().len()) < x {
            min_n = target;
        } else {
            max_n = target;
        }
    }
    let ans = if a * max_n + b * (max_n.to_string().len()) > x {
        min_n
    } else {
        max_n
    };
    println!("{}", ans);
}
