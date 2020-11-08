#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
fn main() {
    input!(n: Chars);

    let mut min = 1000000007;

    for i in 0..1 << n.len() {
        let mut sum = 0;
        let mut count = 0;
        for j in 0..n.len() {
            if (1 << j) & i != 0 {
                count += 1;
                sum += n[j] as i32 - 48;
            }
        }
        if sum % 3 == 0 {
            min = std::cmp::min(min, n.len() - count);
        }
    }

    println!("{}", if min != n.len() { min as isize } else { -1 });
}
