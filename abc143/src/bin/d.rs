#[allow(unused_imports)]
use proconio::{fastout, input};

use std::cmp::Reverse;

#[fastout]
fn main() {
    input!(n: usize, mut l: [isize; n]);
    let mut count = 0;
    l.sort_unstable_by_key(|&x| Reverse(x));

    for i in 0..n {
        for j in i + 1..n {
            let a1 = l[i] - l[j];
            let a2 = l[j] - l[i];
            let a3 = l[i] + l[j];
            for k in j + 1..n {
                if !(l[k] < a3) {
                    break;
                }
                if a1 < l[k] && a2 < l[k] {
                    count += 1;
                }
            }
        }
    }

    println!("{}", count);
}
