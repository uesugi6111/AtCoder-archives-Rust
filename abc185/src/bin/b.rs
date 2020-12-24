#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
fn main() {
    input!(n: i64, m: i64, t: i64, ab: [(i64, i64); m]);

    let mut leave_time = 0;

    let mut battery = n;
    for (aa, bb) in ab {
        battery -= aa - leave_time;
        if battery <= 0 {
            println!("No");
            return;
        }
        battery = std::cmp::min(battery + bb - aa, n);
        leave_time = bb;
    }

    battery -= t - leave_time;
    if battery <= 0 {
        println!("No");
        return;
    }

    println!("Yes");
}
