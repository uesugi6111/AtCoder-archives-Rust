#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
fn main() {
    input!(mut x: usize,  y: usize,a: usize,b: usize);

    let mut count = 0;

    loop {
        let xa = x * a;

        if xa - x > b || xa >= y {
            break;
        }

        count += 1;
        x = xa;
    }

    println!(
        "{}",
        if y == x {
            count
        } else {
            count + (y - x - 1) / b
        }
    );
}
