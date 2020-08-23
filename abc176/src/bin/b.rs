#[allow(unused_imports)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(s: String);

    let mut sum = 0;
    for ss in s.chars() {
        sum += ss as i32 - 48;
    }

    println!("{}", if sum % 9 == 0 { "Yes" } else { "No" });
}
