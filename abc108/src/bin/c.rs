#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
fn main() {
    input!(n: usize, k: usize);

    println!(
        "{}",
        if k % 2 == 0 {
            (1..n + 1).filter(|x| *x % k == 0).count().pow(3)
                + (1..n + 1).filter(|x| *x % k == k / 2).count().pow(3)
        } else {
            (1..n + 1).filter(|x| *x % k == 0).count().pow(3)
        }
    );
}
