#[allow(unused_imports)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(c: char);
    let cc = c as u8 + 1;
    println!("{}", cc as char);
}
