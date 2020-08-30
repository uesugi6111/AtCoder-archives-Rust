use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(s: char);

    println!(
        "{}",
        if s == 'A' {
            'T'
        } else if s == 'T' {
            'A'
        } else if s == 'C' {
            'G'
        } else {
            'C'
        }
    );
}
