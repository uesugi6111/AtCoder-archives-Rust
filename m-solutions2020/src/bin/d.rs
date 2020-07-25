#[allow(unused_imports)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(n: usize, a: [usize; n]);

    let mut money: usize = 1000;

    for i in 0..n - 1 {
        if a[i] < a[i + 1] {
            let kabu = money / a[i];
            money = money % a[i] + kabu * a[i + 1];
        }
    }

    println!("{}", money);
}
