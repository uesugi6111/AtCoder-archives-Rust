#[allow(unused_imports)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(a: usize, b: usize);

    let ans = (a + b) / 2;

    println!(
        "{}",
        if ans * 2 == a + b {
            ans.to_string()
        } else {
            "IMPOSSIBLE".to_string()
        }
    );
}
