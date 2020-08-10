#[allow(unused_imports)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(n: usize, x: usize, l: [usize; n]);

    let mut count = 0;
    let mut d = 0;
    loop {
        if x < d {
            break;
        }
        if count >= n {
            count += 1;
            break;
        }

        d += l[count];

        count += 1;
    }

    println!("{}", count);
}
