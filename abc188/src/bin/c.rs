#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
fn main() {
    input!(n: u32, a: [i64; 2i64.pow(n)]);
    let nn = 2i64.pow(n) as usize;

    let mut l_max: (i64, usize) = (0, 0);
    let mut r_max: (i64, usize) = (0, 0);

    for i in 0..nn / 2 {
        if l_max.0 < a[i] {
            l_max = (a[i], i + 1);
        }
        if r_max.0 < a[i + nn / 2] {
            r_max = (a[i + nn / 2], i + nn / 2 + 1);
        }
    }

    println!("{}", if l_max.0 < r_max.0 { l_max.1 } else { r_max.1 });
}
