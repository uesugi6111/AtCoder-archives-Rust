use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(a: isize, b: isize);

    println!("{}", if b % a == 0 { a + b } else { b - a });
}
