use std::io::{self, BufReader};

use proconio::{input, source::line::LineSource};
fn main() {
    let mut stdin = LineSource::new(BufReader::new(io::stdin()));

    input!(from &mut stdin,n: usize);

    let mut left = 0;
    let mut right = n as i64 + 1;

    while right - left > 1 {
        let mid = left + (right - left) / 2;

        if {
            println!("? {}", mid);
            input!(from &mut stdin,s: i64);
            s == 0
        } {
            left = mid;
        } else {
            right = mid
        };
    }

    println!("! {}", left);
}
