#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
fn main() {
    input!(mut a: [isize;4]);

    let aa = a[1] * a[3];
    let bb = a[0] * a[3];
    let cc = a[1] * a[2];
    let dd = a[0] * a[2];

    println!(
        "{}",
        std::cmp::max(std::cmp::max(aa, bb), std::cmp::max(cc, dd))
    );
}
