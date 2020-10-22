#[allow(unused_imports)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(n: usize, k: usize, mut a: [usize; n]);

    a.sort_unstable();

    let mut ans_vec = Vec::new();

    for i in 0..n {
        ans_vec.push(i);
    }
    let mut left = 0;
    let mut right = a[a.len() - 1];

    let func = |t_max: &usize| -> bool {
        let mut cat_sum = 0;
        for v in a.iter() {
            if v <= t_max {
                continue;
            }
            cat_sum += v / t_max;

            if cat_sum > k {
                return true;
            }
        }

        cat_sum > k
    };
    while right - left > 1 {
        let buff = (left + right) / 2;

        if func(&buff) {
            left = buff;
        } else {
            right = buff;
        }
    }

    println!("{}", right);
}
