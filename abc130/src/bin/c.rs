#[allow(unused_imports)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(w: f64, h: f64, x: f64, y: f64);

    print!("{}", w as f64 * h as f64 / 2f64);
    println!(" {}", if w / 2f64 == x && h / 2f64 == y { 1 } else { 0 });
}
