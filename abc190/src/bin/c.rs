#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
fn main() {
    input!(
        n: usize,
        m: usize,
        ab: [(i64, i64); m],
        k: usize,
        cd: [(i64, i64); k]
    );

    let mut max = 0;

    for i in 0..(1 << k) as usize {
        let mut count = 0;
        let mut v = vec![0; n];

        for j in 0..k {
            if (1 << j) & i == 0 {
                v[(cd[j].0 - 1) as usize] += 1;
            } else {
                v[(cd[j].1 - 1) as usize] += 1;
            }
        }

        for (a, b) in &ab {
            if v[(a - 1) as usize] != 0 && v[(b - 1) as usize] != 0 {
                count += 1;
            }
        }

        max = std::cmp::max(max, count);
    }

    println!("{}", max);
}
