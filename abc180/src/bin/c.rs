#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
fn main() {
    input!(n: usize);
    let div = divisor(n);
    for v in div {
        println!("{}", v);
    }
}

fn divisor(n: usize) -> Vec<usize> {
    let mut ret = vec![];

    for i in 1..(n as f64).sqrt() as usize + 1 {
        if n % i == 0 {
            ret.push(i);
            if i * i != n {
                ret.push(n / i);
            }
        }
    }
    ret.sort();
    ret
}
