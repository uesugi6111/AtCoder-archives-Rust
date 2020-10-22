use num::integer::gcd;
#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
fn main() {
    input!(n: usize ,mut a:[usize;n]);

    let min = a.iter().min().unwrap();
    let mut g = gcd(a[0], *min);
    for v in a.iter().skip(1) {
        g = gcd(g, *v);
    }
    println!("{}", g);
}
