use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(n: usize, a: [usize; n]);
    let mut ra = vec![0usize; n];
    ra[n - 1] = a[n - 1];
    for i in 1..n {
        ra[n - 1 - i] = ra[n - i] + a[n - 1 - i];
    }
    for i in 0..n {
        ra[i] = (ra[i] - a[i]) % 1_000_000_007;
    }

    let mut sum: usize = 0;
    for i in 0..n {
        sum += a[i] * ra[i] % 1_000_000_007;
        sum %= 1_000_000_007;
    }

    println!("{}", sum % 1_000_000_007);
}
