#[allow(unused_imports)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(n: usize, h: [usize; n]);

    let mut buff = h[n - 1];
    for &v in h.iter().rev() {
        if buff + 1 == v || buff == v {
            continue;
        }
        if buff > v {
            buff = v;
            continue;
        }
        println!("{}", "No");
        return;
    }

    println!("{}", "Yes");
}
