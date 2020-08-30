use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(n: usize, l: [usize; n]);

    let max = l.iter().max().unwrap();

    let sum = l.iter().sum();

    println!("{}", if 2 * max < sum { "Yes" } else { "No" });
}
