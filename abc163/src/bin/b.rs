use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(n: usize, m: usize, a: [usize; m]);

    let sum = a.iter().sum::<usize>();
    let ans = if n < sum { -1 } else { (n - sum) as i64 };
    println!("{}", ans);
}
