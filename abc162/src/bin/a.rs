use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(n: String);

    let vs: Vec<char> = n.chars().collect();

    println!("{}", if vs.contains(&'7') { "Yes" } else { "No" });
}
