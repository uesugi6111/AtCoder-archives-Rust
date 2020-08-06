#[allow(unused_imports)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(a: usize, b: usize);

    let mut count = 0;
    let mut mouth = 1;
    loop {
        if mouth >= b {
            break;
        }
        mouth += a - 1;
        count += 1;
    }

    println!("{}", count);
}
