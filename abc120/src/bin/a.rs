use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(a: usize, b: usize, c: usize);

    let count = std::cmp::min(b / a, c);

    println!("{}", count);
}
