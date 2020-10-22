#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
fn main() {
    input!(n: usize, a: [u128; n]);

    let mut aaa: u128 = 1;

    if a.iter().any(|x| *x == 0) {
        print!("0");
        return;
    }
    for v in a {
        if v * aaa > 1_000_000_000_000_000_000 {
            print!("-1");
            return;
        }

        aaa *= v;
    }

    println!("{}", aaa);
}
