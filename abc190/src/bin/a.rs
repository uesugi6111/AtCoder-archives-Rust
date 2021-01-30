#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
fn main() {
    input!(a: usize, b: usize, c: usize);

    println!(
        "{}",
        if a < b {
            "Aoki"
        } else if a == b {
            if c == 1 {
                "Takahashi"
            } else {
                "Aoki"
            }
        } else {
            "Takahashi"
        }
    );
}
