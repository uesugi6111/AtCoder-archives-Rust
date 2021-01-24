#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
fn main() {
    input!(n: usize, s: [String; n]);

    let mut count = 1;

    for (i, ss) in s.iter().enumerate() {
        if ss == "OR" {
            count += 2i64.pow(i as u32 + 1);
        }
    }

    println!("{}", count);
}
