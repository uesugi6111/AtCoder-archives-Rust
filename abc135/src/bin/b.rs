#[allow(unused_imports)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(n: usize, mut p: [usize; n]);
    let pp = p.clone();
    p.sort();

    let mut count = 0_usize;
    for i in 0..n {
        if p[i] != pp[i] {
            count += 1;
        }
    }

    println!("{}", if count <= 2 { "YES" } else { "NO" });
}
