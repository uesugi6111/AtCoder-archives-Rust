#[allow(unused_imports)]
use proconio::{
    input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
fn main() {
    input!(n: usize);
    println!("{}", if n == 12 { 1 } else { n + 1 });
}
