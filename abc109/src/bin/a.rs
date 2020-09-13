use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(a: usize, b: usize);

    println!("{}", if (a * b) % 2 != 0 { "Yes" } else { "No" });
}
