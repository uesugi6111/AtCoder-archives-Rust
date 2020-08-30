use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(n: usize, t: f64, a: f64, h: [f64; n]);

    let mut min = 100_000_000.0;
    let mut ans = 0;
    for i in 0..h.len() {
        let sa = (a - (t - h[i] * 0.006)).abs();

        if min >= sa {
            min = sa;
            ans = i;
        }
    }

    println!("{}", ans + 1);
}
