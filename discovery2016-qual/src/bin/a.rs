#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
fn main() {
    input!(w: usize);

    let ss = "DiscoPresentsDiscoveryChannelProgrammingContest2016";

    let s: Vec<_> = ss.chars().collect();

    for i in 0..s.len() {
        print!("{}", s[i]);
        if i != s.len() - 1 && (i + 1) % w == 0 {
            println!();
        }
    }
    println!();
}
