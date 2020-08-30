use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(n: [usize; 5]);

    let mut min = 10;

    let mut sum = 0;
    for v in &n {
        let d_v = *v % 10;

        if d_v != 0 {
            min = std::cmp::min(min, d_v);
        }

        sum += *v + (10 - d_v) % 10;
    }
    println!("{}", sum - (10 - min) % 10);
}
