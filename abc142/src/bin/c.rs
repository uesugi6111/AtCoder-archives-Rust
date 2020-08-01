#[allow(unused_imports)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(n: usize, a: [usize; n]);
    let mut ar: Vec<Vec<usize>> = vec![];

    for i in 0..n {
        let array: Vec<usize> = vec![a[i], i];
        ar.push(array);
    }

    ar.sort();

    for v in ar {
        println!("{}", v[1] + 1);
    }
}
