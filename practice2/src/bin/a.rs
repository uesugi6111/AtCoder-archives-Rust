use itertools_num::ItertoolsNum;
#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
fn main() {
    input!(n: usize, k: usize, a: [i64; n]);

    let ruiseki_wa = a.iter().cumsum().collect::<Vec<i64>>();

    let mut sum: i64 = (0..n - k).map(|x| ruiseki_wa[x + k] - ruiseki_wa[x]).sum();
    sum += ruiseki_wa[k - 1];

    println!("{}", sum);
}
