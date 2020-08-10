#[allow(unused_imports)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(n: usize, ab: [[usize; 2]; n]);

    let mut count = 0;

    for v in ab.iter().rev() {
        count += (v[1] - (count + v[0]) % v[1]) % v[1];
    }

    println!("{}", count);
}
