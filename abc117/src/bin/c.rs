#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
fn main() {
    input!(n: usize,m:usize,mut x:[isize;m]);

    x.sort();
    let mut x_m = Vec::<isize>::new();
    for i in 1..x.len() {
        x_m.push(x[i] - x[i - 1]);
    }

    x_m.sort();
    let ans = (0..if x_m.len() < (n - 1) {
        0
    } else {
        x_m.len() - (n - 1)
    })
        .map(|x| x_m[x])
        .sum::<isize>();

    println!("{}", ans);
}
