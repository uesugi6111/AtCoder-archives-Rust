#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
fn main() {
    input!(k: usize);

    let mut count = 0;
    for i in 1..k {
        for j in i + 1..k + 1 {
            if (i + j) % 2 != 0 {
                count += 1;
            }
        }
    }

    println!("{}", count);
}
