#[allow(unused_imports)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(n: usize, d: usize, x: [[isize; d]; n]);

    let mut ar = Vec::new();
    for i in 0..200 {
        ar.push(i * i);
    }

    let mut count = 0;

    for i in 0..n {
        for j in i + 1..n {
            let mut sum = 0;
            for k in 0..d {
                sum += (x[i][k] - x[j][k]) * (x[i][k] - x[j][k]);
            }
            if ar.contains(&sum) {
                count += 1;
            }
        }
    }
    println!("{}", count);
}
