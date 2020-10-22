#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
fn main() {
    input!(x: [isize; 4]);

    let x_sub = x[0] - x[2];
    let y_sub = x[1] - x[3];

    let x3 = x[2] + y_sub;
    let y3 = x[3] - x_sub;
    let x4 = x3 + x_sub;
    let y4 = y3 + y_sub;

    println!("{} {} {} {}", x3, y3, x4, y4);
}
