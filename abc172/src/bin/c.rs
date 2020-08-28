#[allow(unused_imports)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(n: usize, m: usize, k: usize, a: [usize; n], b: [usize; m]);

    let mut max_books = 0;
    let mut a_count = 0;
    let mut a_sum = 0;

    let mut ruiseki = Vec::new();
    ruiseki.push(0);
    for i in 0..m {
        ruiseki.push(ruiseki[i] + b[i]);
    }

    for i in 0..n + 1 {
        //累積和から謎の処理
        // 二分探索
        let mut l = 0;
        let mut r = m + 1;
        while (r as isize - l as isize).abs() > 1 {
            let lr = (l + r) / 2;
            if ruiseki[lr] + a_sum <= k {
                l = lr;
            } else {
                r = lr;
            }
        }

        if ruiseki[l] + a_sum <= k {
            max_books = std::cmp::max(max_books, a_count + l);
        }
        if i == n {
            break;
        }
        a_sum += a[i];
        a_count += 1;
    }

    println!("{}", max_books);
}
