#[allow(unused_imports)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(a: usize);

    println!("{}", a + a * a + a * a * a);
}
