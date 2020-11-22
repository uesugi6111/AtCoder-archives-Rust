#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
fn main() {
    input!(n: usize, x: isize, s: Chars);

    let mut score = x;
    for ss in s {
        if ss == 'o' {
            score += 1;
        } else {
            score = std::cmp::max(0, score - 1);
        }
    }
    println!("{}", score);
}
