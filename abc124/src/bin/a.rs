use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(a: usize, b: usize);
    let mut ab = vec![a, b, std::cmp::max(a, b) - 1];
    ab.sort();
    println!("{}", ab[2] + ab[1]);
}
