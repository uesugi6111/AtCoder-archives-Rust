use itertools::*;
#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
fn main() {
    input!(b: i64, c: i64);
    let ans: i64 = if b >= c {
        1 + if c > 1 { 1 } else { 0 } + ((c - 1) / 2) + c / 2
    } else if b.abs() < c {
        1+ b.abs() + if b <= 0 { (c - 1) / 2 } else { (c - 2) / 2 }
         //   - (b.abs() - if b <= 0 { (c - 2) / 2 } else { (c - 1) / 2 })
            + b.abs()
            + if b <= 0 { c / 2 } else { (c - 1) / 2 }
    //  - (b.abs() - if b <= 0 { (c - 1) / 2 } else { c / 2 })
    } else {
        1 + b.abs() + if b <= 0 { (c - 1) / 2 } else { (c - 2) / 2 }
            - (b.abs() - if b <= 0 { (c - 2) / 2 } else { (c - 1) / 2 })
            + b.abs()
            + if b <= 0 { c / 2 } else { (c - 1) / 2 }
            - (b.abs() - if b <= 0 { (c - 1) / 2 } else { c / 2 })
    };
    let data = vec![
        (0, 1.),
        (1, 1.),
        (0, 2.),
        (0, 3.),
        (1, 3.),
        (1, 2.),
        (2, 2.),
    ];
    let data = vec![10, 20, 30, 20, 40, 10, 50];
    itertools::assert_equal(data.into_iter().unique(), vec![10, 20, 30, 40, 50]);
    println!("{}", ans);
}
