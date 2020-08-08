#[allow(unused_imports)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(l: usize, r: usize);
    let start = l % 2019;
    let end = start + if r - l > 2019 { 2019 } else { r - l };

    let mut ar = Vec::new();
    for i in start..end + 1 {
        ar.push(i);
    }

    let mut ans_min = 1 << 20;
    for i in 0..ar.len() {
        for j in i + 1..ar.len() {
            let v = (ar[i] * ar[j]) % 2019;
            if v < ans_min {
                ans_min = v;
            }
        }
    }

    println!("{}", ans_min);
}
