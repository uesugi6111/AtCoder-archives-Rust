#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
fn main() {
    input!(n: usize, m: usize, a: [i64; n]);

    let mut aa = std::collections::BinaryHeap::from(a);

    for _ in 0..m {
        let p = aa.pop().unwrap();
        aa.push(p / 2);
    }

    println!("{}", aa.iter().sum::<i64>());
}
