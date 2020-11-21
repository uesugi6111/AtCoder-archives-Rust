#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
fn main() {
    input!(s: isize, p: isize);

    for m in 0..=s {
        let ans = m * (s - m);
        if ans == p {
            println!("Yes");
            return;
        }
        if ans > p {
            break;
        }
    }

    println!("No");
}
