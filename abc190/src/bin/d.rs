use std::usize;

#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
fn main() {
    input!(n: usize);

    let div = enum_divisors(n);

    println!("{}", div.iter().filter(|x| *x % 2 == 1).count() * 2);
}

pub fn enum_divisors(n: usize) -> Vec<usize> {
    let mut res = vec![];
    for i in 1..(n as f64).sqrt() as usize + 1 {
        if n % i == 0 {
            res.push(i);
            if n / i != i {
                res.push(n / i);
            }
        }
    }
    res
}
