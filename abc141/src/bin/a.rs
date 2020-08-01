#[allow(unused_imports)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(s: String);

    let ss = vec!["Sunny", "Cloudy", "Rainy"];
    for i in 0..3 {
        if ss[i] == s {
            println!("{}", ss[(i + 1) % 3]);
        }
    }
}
