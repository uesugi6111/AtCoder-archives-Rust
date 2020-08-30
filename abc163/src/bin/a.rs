use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(r: usize);

    println!("{}", (r * 2) as f32 * std::f32::consts::PI);
}
