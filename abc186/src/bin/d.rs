#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
fn main() {
    input!(n: usize, mut a: [i64; n]);

    a.sort();

    let mut ruiseki_wa = vec![];

    ruiseki_wa.push(a[0]);

    for i in 1..n {
        ruiseki_wa.push(ruiseki_wa[i - 1] + a[i]);
    }

    let mut sum = 0;
    for i in 1..n {
        sum += (a[i] * i as i64 - ruiseki_wa[i - 1]).abs();
    }

    println!("{}", sum);
}
