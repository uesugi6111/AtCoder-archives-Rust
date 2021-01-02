#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
fn main() {
    input!(n: usize, a: Chars, b: Chars, c: Chars);

    let mut count = 0;
    for i in 0..n {
        if a[i] == b[i] && a[i] == c[i] {
            continue;
        }
        if a[i] != b[i] && a[i] != c[i] && b[i] != c[i] {
            count += 2;
            continue;
        }
        count += 1;
    }

    println!("{}", count);
}
