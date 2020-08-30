use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(h: usize, w: usize, hh: usize, ww: usize);

    println!("{}", h * w - (w * hh + h * ww - ww * hh));
}
