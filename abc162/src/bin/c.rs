use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(n: usize);
    let mut sum = 0;
    for i in 1..n + 1 {
        for j in 1..n + 1 {
            for k in 1..n + 1 {
                sum += num::integer::gcd(i, num::integer::gcd(j, k));
            }
        }
    }
    println!("{}", sum);
}
