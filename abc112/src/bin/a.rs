use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(n: usize);

    println!(
        "{}",
        if n == 1 {
            "Hello World".to_string()
        } else {
            input!(a: usize, b: usize);
            (a + b).to_string()
        }
    );
}
