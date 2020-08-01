#[allow(unused_imports)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(_n: usize, s: String);

    let mut count = 0;
    let mut buff = 'A';
    for sv in s.chars() {
        if buff != sv {
            buff = sv;
            count += 1;
        }
    }

    println!("{}", count);
}
