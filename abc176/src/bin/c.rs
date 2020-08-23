#[allow(unused_imports)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(n: usize, a: [usize; n]);

    let mut step = 0;
    let mut now = a[0];
    for v in &a {
        if now <= *v {
            now = *v;
            continue;
        }

        let buff = now - v;
        step += buff;
    }

    println!("{}", step);
}
