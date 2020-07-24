#[allow(unused_imports)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(h: i64, n: i64, mut a: [i64; n]);

    let ans = if a.iter().sum::<i64>() >= h {
        "Yes"
    } else {
        "No"
    };

    println!("{}", ans);
}
