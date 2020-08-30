use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(n: usize);
    let ans: usize = (0usize..n).filter(|x| x % 3 != 0 && x % 5 != 0).sum();

    println!("{}", ans);
}
