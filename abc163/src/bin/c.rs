use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(n: usize, a: [usize; n - 1]);
    let mut a_v = vec![0; n];
    for v in a {
        a_v[v - 1] += 1;
    }

    let ans = a_v.iter().take(n).join("\n");
    print!("{}", ans);
}
