#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
fn main() {
    input!(n: usize, a: [i64; n], b: [i64; n]);

    let mut a_max = 0;
    let mut aa = vec![];
    for i in 0..n {
        a_max = std::cmp::max(a_max, a[i]);
        aa.push(a_max);
    }

    let mut c = vec![];
    let mut buff = 0;
    for i in 0..n {
        let mut max = 0;
        max = std::cmp::max(max, b[i] * aa[i]);

        max = std::cmp::max(buff, max);
        c.push(max);
        buff = max;
    }

    for v in c {
        println!("{}", v);
    }
}
