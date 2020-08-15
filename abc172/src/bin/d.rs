#[allow(unused_imports)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(n: usize);

    let mut ans = 0;

    for i in 1..(n + 1) {
        let a = n / i;
        ans += (((2 * i) + (a - 1) * i) * a) / 2;
    }

    println!("{}", ans);
}
