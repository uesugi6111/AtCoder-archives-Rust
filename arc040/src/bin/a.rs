#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
fn main() {
    input!(n: usize, s: [Chars; n]);

    let mut a = vec![0; 2];
    for i in s {
        for j in i {
            if j == 'R' {
                a[0] += 1;
            }
            if j == 'B' {
                a[1] += 1;
            }
        }
    }
    println!(
        "{}",
        if a[0] == a[1] {
            "DRAW"
        } else if a[0] < a[1] {
            "AOKI"
        } else {
            "TAKAHASHI"
        }
    );
}
