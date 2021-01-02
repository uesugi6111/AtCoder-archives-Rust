#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
fn main() {
    input!(c: [[char; 4]; 4]);

    for i in 0..4 {
        for j in 0..4 {
            print!("{}", c[3 - i][3 - j]);
            if j == 3 {
                println!();
            } else {
                print!(" ");
            }
        }
    }
}
