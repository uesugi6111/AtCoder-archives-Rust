#[allow(unused_imports)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(n: usize, k: usize, s: String);

    let sss: Vec<char> = s.chars().collect();
    for i in 0..s.len() {
        if i == k - 1 {
            print!("{}", sss[i].to_ascii_lowercase());
        } else {
            print!("{}", sss[i]);
        }
    }
}
