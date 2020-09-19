#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
fn main() {
    input!(n: usize);

    let mut count = 0;
    for i in 1..n {
        count += (n - 1) / i;
    }
    println!("{}", count);
}
