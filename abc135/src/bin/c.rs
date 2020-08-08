#[allow(unused_imports)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(n: usize, a: [usize; n + 1], b: [usize; n]);

    let mut cap = 0;

    let mut count = 0;

    for i in 0..n {
        count += if a[i] < b[i] + cap { a[i] } else { b[i] + cap };

        cap = if a[i] < b[i] + cap {
            if a[i] > cap {
                b[i] - (a[i] - cap)
            } else {
                b[i]
            }
        } else {
            0
        };
    }

    count += if a[n] < cap { a[n] } else { cap };

    println!("{}", count);
}
