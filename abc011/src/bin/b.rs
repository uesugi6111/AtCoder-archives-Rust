#[allow(unused_imports)]
use proconio::{
    input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
fn main() {
    input!(s: Chars);
    print!("{}", s[0].to_string().to_uppercase());

    s.iter()
        .skip(1)
        .map(|x| x.to_string().to_lowercase())
        .for_each(|x| print!("{}", x));
    println!();
}
