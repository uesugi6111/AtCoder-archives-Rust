use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(d: f64, t: f64, s: f64);

    println!("{}", if d / s <= t { "Yes" } else { "No" });
}
