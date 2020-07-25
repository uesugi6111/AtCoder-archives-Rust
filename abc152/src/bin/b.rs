#[allow(unused_imports)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(a: u64, b: u64);
    let mut ans: String = "".to_string();
    if a > b {
        for i in 0..a {
            ans.push(std::char::from_digit(b as u32, 10).unwrap());
        }
    } else {
        for i in 0..b {
            ans.push(std::char::from_digit(a as u32, 10).unwrap());
        }
    }

    println!("{}", ans);
}
