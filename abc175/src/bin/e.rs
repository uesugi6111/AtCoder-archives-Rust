#[allow(unused_imports)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(n: usize, _k: usize, p: [isize; n], c: [isize; n]);

    let mut pc = Vec::new();

    for i in 0..n {
        pc.push(vec![p[i], c[i]]);
    }
    pc.sort();

    println!("{:?}", &pc);
}
