use proconio::{fastout, input};
use std::str::FromStr;

#[fastout]
fn main() {
    input!(n: usize, s: [[String; 2]; n]);

    let mut sum = 0.0;

    for v in s {
        if &v[1] == "JPY" {
            sum += (&v[0]).parse::<usize>().unwrap() as f64;
        } else {
            sum += f64::from_str(&v[0]).unwrap() * 380_000.0;
        }
    }

    println!("{}", sum);
}
