#[allow(unused_imports)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(n: usize,mut w: [usize; n]);

    let mut vec_f = Vec::new();
    let mut vec_b = Vec::new();

    vec_f.push(w[0]);
    for i in 0..n - 1 {
        vec_f.push(vec_f[i] + w[i + 1]);
    }

    vec_b.push(w[n - 1]);
    for i in 0..n - 1 {
        vec_b.push(vec_b[i] + w[n - i - 1 - 1]);
    }

    let mut min: usize = 1000000000usize;
    for i in 0..n - 1 {
        min = ch_min(
            min,
            (vec_f[i] as isize - vec_b[n - 2 - i] as isize).abs() as usize,
        );
    }

    println!("{}", min);
}
fn ch_min(a: usize, b: usize) -> usize {
    return if a < b { a } else { b };
}
