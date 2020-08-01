#[allow(unused_imports)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(s: String);
    let v: Vec<char> = s.chars().collect();

    for i in 0..s.len() {
        if i % 2 == 0 {
            if v[i] == 'L' {
                println!("{}", "No");
                return;
            }
        } else {
            if v[i] == 'R' {
                println!("{}", "No");
                return;
            }
        }
    }
    println!("{}", "Yes");
}
