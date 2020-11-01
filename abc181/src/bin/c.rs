#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
fn main() {
    input!(n: usize, xy: [(isize, isize); n]);

    for i in 0..n {
        for j in i + 1..n {
            let xx = xy[j].0 - xy[i].0;
            let yy = xy[j].1 - xy[i].1;

            for k in j + 1..n {}
        }
    }

    println!("{}", n);
}
