#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
fn main() {
    input!(c: Chars);
    println!(
        "{}",
        if c[0] == c[1] && c[1] == c[2] {
            "Won"
        } else {
            "Lost"
        }
    );
}
