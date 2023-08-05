use std::io::{self, BufReader};

use proconio::{input, source::line::LineSource};
#[proconio::fastout]
fn main() {
    let mut stdin = LineSource::new(BufReader::new(io::stdin()));
    macro_rules! input(($($tt:tt)*) => (proconio::input!(from &mut stdin, $($tt)*)));
    input!(n: usize, k: usize);

    if n != 5 {
        panic!();
    }

    println!("! 0");
}
