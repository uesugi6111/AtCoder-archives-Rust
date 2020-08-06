#[allow(unused_imports)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(n: usize, h: [usize; n]);

    let mut count = 0;
    let mut max = 0;
    for i in 0..h.len() - 1 {
        if h[i] >= h[i + 1] {
            count += 1;
            if max < count {
                max = count;
            }
        } else {
            count = 0;
        }
    }

    println!("{}", max);
}
