#[allow(unused_imports)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(n: u64, k: u64, a: [u64; n]);

    for i in 0..(n - k) {
        if a[i as usize] < a[(i + k) as usize] {
            println!("{}", "Yes");
        } else {
            println!("{}", "No");
        }
    }
}
