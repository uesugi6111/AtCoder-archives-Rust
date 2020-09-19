#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
fn main() {
    input!(x: usize);
    let mut n = 100;
    let mut count = 0;
    while n < x {
        n += n / 100;
        count += 1;
    }

    println!("{}", count);
}
