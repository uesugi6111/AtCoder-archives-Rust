#[allow(unused_imports)]
use proconio::{fastout, input};
use std::collections::HashSet;

// DP勉強してからときなおす
#[fastout]
fn main() {
    input!(n: usize, m: usize, a: [usize; m]);
    let a_set: HashSet<_> = a.iter().cloned().collect();

    let mut count = 1;
    for i in (1..n + 1).rev() {
        if a_set.contains(&i) {
            continue;
        }
        let flag1 = !a_set.contains(&(i - 1));
        let flag2 = i != 1 && !a_set.contains(&(i - 2));

        if flag1 && flag2 {
            count = (count * 2) % (1e9 as usize + 7) as usize;
        } else if flag1 || flag2 {
        } else {
            count = 0;
        }
    }

    println!("{}", count);
}
