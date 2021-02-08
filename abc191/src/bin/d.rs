#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
fn main() {
    input!(x: f64, y: f64, r: f64);

    let adjust = 10000f64;

    let xx = (x * adjust).round() as i64;
    let yy = (y * adjust).round() as i64;
    let rr = (r * adjust).round() as i64;

    let mut count = 0;

    for rrr in (-rr / adjust as i64)..=(rr / adjust as i64) {
        let xxx = (rrr * adjust as i64 - xx).abs();
        if xxx > rr {
            continue;
        }
        let yyy = dbg!(sqrt(rr.pow(2) - xxx.pow(2)));
        count += 2 * yyy / adjust as i64;
    }
    println!("{}", count);
}

fn sqrt(num: i64) -> i64 {
    (num as f64).sqrt() as i64
}
