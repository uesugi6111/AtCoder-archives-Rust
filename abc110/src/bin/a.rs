use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(n: [usize; 3]);

    let &max = n.iter().max().unwrap();
    let sum: usize = n.iter().sum();

    println!("{}", max * 9usize + sum);
}
