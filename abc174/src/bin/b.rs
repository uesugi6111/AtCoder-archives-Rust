#[allow(unused_imports)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(n: usize, d: isize, xy: [[isize; 2]; n]);

    let mut count = 0;
    for v in xy {
        if d * d >= v[0] * v[0] + v[1] * v[1] {
            count += 1;
        }
    }

    println!("{}", count);
}
