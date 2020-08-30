use proconio::*;
use std::iter::FromIterator;

#[fastout]
fn main() {
    input!(n: marker::Chars);

    let aaa: Vec<char> = n
        .iter()
        .map(|&x| if x == '1' { '9' } else { '1' })
        .collect();

    println!("{}", String::from_iter(aaa));
}
