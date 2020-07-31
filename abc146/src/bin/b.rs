#[allow(unused_imports)]
use proconio::{fastout, input};
use std::char;
#[fastout]
fn main() {
    input!(n: u32, s: String);
    for ss in s.chars() {
        print!(
            "{}",
            char::from_u32((ss as u32 + n - 65) % 26 + 65).unwrap()
        );
    }
}
