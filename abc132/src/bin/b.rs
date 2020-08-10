#[allow(unused_imports)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(n: usize, p: [usize; n]);
    let mut count = 0;
    for i in 1..n - 1 {
        let mut v = vec![p[i - 1], p[i], p[i + 1]];
        v.sort();
        if v[1] == p[i] {
            count += 1;
        }
    }
    println!("{}", count);
}
