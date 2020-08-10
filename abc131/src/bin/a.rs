#[allow(unused_imports)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(s: String);

    let char_vec: Vec<char> = s.chars().collect();
    for i in 0..s.len() - 1 {
        if char_vec[i] == char_vec[i + 1] {
            println!("{}", "Bad");
            return;
        }
    }
    println!("{}", "Good");
}
