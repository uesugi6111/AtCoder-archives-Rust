#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
fn main() {
    input!(s: Chars);

    for i in 0..s.len() {
        if s[i].is_digit(10) {
            if s.len() - 1 >= i + 1 && s[i + 1].is_digit(10) {
                print!("{}", s[i]);
                println!("{}", s[i + 1]);

                return;
            }
            println!("{}", s[i]);
            return;
        }
    }
}
