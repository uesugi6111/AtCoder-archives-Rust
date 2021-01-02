#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
fn main() {
    input!(xy: [(i64, i64); 2]);

    println!(
        "{}",
        (xy[0].0 - xy[1].0).abs() + (xy[0].1 - xy[1].1).abs() + 1
    );
}
