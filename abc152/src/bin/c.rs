#[allow(unused_imports)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(n: u64, p: [u64; n]);
    let mut min: u64 = 0;
    let mut count: u64 = 0;
    for &pp in p.iter() {
        min = if min < pp { min } else { pp };
        if min == pp {
            count += 1;
        }
    }
    println!("{}", count);
}
