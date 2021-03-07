#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
fn main() {
    input!(t: usize, lr: [(i64, i64); t]);

    for (l, r) in lr {
        if l == r {
            if l == 0 {
                println!("{}", 1);
                continue;
            }
            println!("{}", 0);
            continue;
        } else if 2 * l > r {
            println!("{}", 0);
            continue;
        }
        println!("{}", (r - l * 2 + 1) * (1 + r - l * 2 + 1) / 2);
    }
}
