use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(n: usize);

    println!(
        "{}",
        (1..10)
            .map(|x| x * 100 + x * 10 + x)
            .filter(|&x| x >= n)
            .min()
            .unwrap()
    );
}
