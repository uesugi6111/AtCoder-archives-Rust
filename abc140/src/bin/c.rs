#[allow(unused_imports)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(n: usize, b: [usize; n - 1]);

    let mut sum = 0;
    for i in 1..n - 1 {
        sum += if b[i] > b[i - 1] { b[i - 1] } else { b[i] };
    }
    sum += b[0];
    sum += b[n - 2];

    println!("{}", sum);
}
