#[allow(unused_imports)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(n: usize, v: [isize; n], c: [isize; n]);

    let mut ans = 0;
    for i in 0..n {
        if 0 < v[i] - c[i] {
            ans += v[i] - c[i];
        }
    }

    println!("{}", ans);
}
