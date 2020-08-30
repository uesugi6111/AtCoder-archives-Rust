use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(n: usize, p: [usize; n]);

    let max: &usize = p.iter().max().unwrap();

    println!("{}", p.iter().sum::<usize>() - *max / 2);
}
