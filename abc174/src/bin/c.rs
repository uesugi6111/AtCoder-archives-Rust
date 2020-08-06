#[allow(unused_imports)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(k: usize);
    let mut count = 1;
    let mut temp = 7;

    while count < (10 as usize).pow(6) {
        temp %= k;
        if temp == 0 {
            println!("{}", count);
            return;
        }
        count += 1;
        temp = temp * 10 + 7;
    }
    println!("{}", -1);
}
