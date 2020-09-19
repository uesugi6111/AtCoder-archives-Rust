#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
fn main() {
    input!(s: Chars);

    for v in &s {
        print!("{}", v);
    }
    if s[s.len() - 1] == 's' {
        println!("es");
    } else {
        println!("s");
    }
}
