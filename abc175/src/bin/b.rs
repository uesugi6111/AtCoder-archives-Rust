#[allow(unused_imports)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(n: usize,mut  l: [usize; n]);

    let mut count = 0;
    for i in 0..n {
        for j in i + 1..n {
            if l[i] == l[j] {
                continue;
            }
            for k in j + 1..n {
                if l[i] == l[k] || l[j] == l[k] {
                    continue;
                }
                if l[i] + l[j] <= l[k] || l[k] + l[i] <= l[j] || l[j] + l[k] <= l[i] {
                    continue;
                }
                count += 1;
            }
        }
    }

    println!("{}", count);
}
