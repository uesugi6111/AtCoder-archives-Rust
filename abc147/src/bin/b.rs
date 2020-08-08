#[allow(unused_imports)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(s: String);

    let mut count = 0;
    for i in 0..s.len() / 2 {
        if s.chars().nth(i) != s.chars().nth(s.len() - 1 - i) {
            count += 1;
        }
    }

    println!("{}", count);
}
