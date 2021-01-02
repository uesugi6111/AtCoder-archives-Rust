use num_integer::Roots;
#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
fn main() {
    input!(a: usize);

    for i in 0..a.sqrt() as usize + 1 {
        if i * i * i == a {
            println!("YES");
            return;
        }
    }

    println!("NO");
}
