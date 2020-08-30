use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(n: usize);

    println!(
        "{}",
        if n == 3 || n == 5 || n == 7 {
            "YES"
        } else {
            "NO"
        }
    );
}
