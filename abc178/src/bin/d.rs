#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};

const MMMOD: usize = 1_000_000_007;
#[fastout]
fn main() {
    input!(s: usize);

    let mut count = 0;

    for i in 1..2000 {
        if s / i < 3 {
            break;
        }
        let mut x = i;
        let mut buff = 1;
        while x > 0 {
            if s < 3 * (x + 1) {
                break;
            }

            buff = (buff * (s - 3 * (x + 1) + 1)) % MMMOD;
            x -= 1;
        }
        count = (count + buff) % MMMOD;
    }
    println!("{}", count);
}
