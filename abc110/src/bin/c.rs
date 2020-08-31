use my_crate::algorithm::eratosthenes;

use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(n: usize);
    let s = eratosthenes::sieve(n);

    println!("{}", s.len());
}
