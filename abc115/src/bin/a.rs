use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(n: usize);

    println!(
        "{}",
        if n == 25 {
            "Christmas"
        } else if n == 24 {
            "Christmas Eve"
        } else if n == 23 {
            "Christmas Eve Eve"
        } else {
            "Christmas Eve Eve Eve"
        }
    );
}
