use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(x: usize, y: usize);

    println!("{}", x + y / 2);
}
