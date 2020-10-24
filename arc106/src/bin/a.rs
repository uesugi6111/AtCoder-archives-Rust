#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
fn main() {
    input!(n: usize);

    let max_a = (n as f64).log(3f64);
    let max_b = (n as f64).log(5f64);

    for i in 1..(max_a as usize) + 2 {
        for j in 1..max_b as usize + 2 {
            if 3usize.pow(i as u32) + 5usize.pow(j as u32) == n {
                println!("{} {}", i, j);
                return;
            }
        }
    }
    println!("-1");
}
