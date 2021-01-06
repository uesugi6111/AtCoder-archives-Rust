#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
fn main() {
    input!(a: Chars, b: Chars);

    let aa: i32 = a.iter().map(|x| *x as i32 - 48).sum();
    let bb: i32 = b.iter().map(|x| *x as i32 - 48).sum();
    println!("{}", std::cmp::max(aa, bb));
}
