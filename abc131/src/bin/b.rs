#[allow(unused_imports)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(n: usize, l: isize);
    let mut vec_a = Vec::new();
    for i in l..(n as isize) + l {
        vec_a.push(vec![i * i, i]);
    }

    vec_a.sort();

    let mut sum = 0;
    for v in &vec_a {
        sum += v[1] as isize;
    }

    println!("{}", sum - vec_a[0][1]);
}
