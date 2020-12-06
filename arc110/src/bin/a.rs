#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
fn main() {
    input!(_n: u128);

    let table = vec![2, 3, 2, 5, 7, 2, 3, 11, 13, 2, 17, 19, 23, 5, 3, 29];
    let mut ans: u128 = 1;

    for i in table {
        ans *= i;
    }

    println!("{}", ans + 1);
}
