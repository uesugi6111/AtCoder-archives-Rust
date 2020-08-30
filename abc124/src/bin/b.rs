use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(n: usize, h: [i64; n]);

    let mut count = 0;
    let mut max = -1;
    for v in h {
        if max <= v {
            max = v;
            count += 1;
        }
    }
    println!("{}", count);
}
