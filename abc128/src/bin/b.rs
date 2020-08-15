#[allow(unused_imports)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(n: usize, sp: [(String, usize); n]);

    let mut aaa = Vec::new();

    for i in 0..n {
        aaa.push(((&sp[i].0, sp[i].1), i));
    }
    aaa.sort_by(|a, b| {
        if (a.0).0 != (b.0).0 {
            (b.0).0.cmp((&a.0).0)
        } else {
            (a.0).1.cmp(&((b.0).1))
        }
    });

    for v in aaa.iter().rev() {
        println!("{}", (v.1) + 1);
    }
}
