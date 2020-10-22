use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(n: usize);
    let ans: usize = (1usize..n + 1).filter(|x| x % 3 != 0 && x % 5 != 0).sum();

    println!("{}", ans);
}
