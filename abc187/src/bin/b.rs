#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
fn main() {
    input!(n: usize, mut xy: [(i64, i64); n]);
    xy.sort_by_key(|x| -x.0);

    let mut count = 0;
    for i in 0..n {
        for j in i + 1..n {
            if xy[i].0 - xy[j].0 == 0 {
                continue;
            }
            let buf = (xy[i].1 as f64 - xy[j].1 as f64) / (xy[i].0 as f64 - xy[j].0 as f64);
            if buf <= 1.0 && buf >= -1.0 {
                count += 1;
            }
        }
    }
    println!("{}", count);
}
