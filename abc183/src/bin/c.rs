use itertools::Itertools;
#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
fn main() {
    input!(n: usize, k: usize, t: [[usize; n]; n]);

    let mut count = 0;
    for per in (1..n).permutations(n - 1) {
        let s = per.len();
        let mut sum = 0;
        sum += t[0][per[0]];
        for i in 0..s - 1 {
            sum += t[per[i]][per[i + 1]];
        }
        sum += t[per[s - 1]][0];

        if sum == k {
            count += 1;
        }
    }

    println!("{}", count);
}
