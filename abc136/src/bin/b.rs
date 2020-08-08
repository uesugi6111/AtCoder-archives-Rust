#[allow(unused_imports)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(n: usize);
    let mut count = 0;
    let mut div = 1;
    loop {
        if n / div >= 1 {
            if n < div * 10 {
                count += (n % ((div) * 10)) - div + 1;
            } else {
                count += div * 9;
            }
        } else {
            break;
        }
        div *= 100;
    }
    println!("{}", count);
}
