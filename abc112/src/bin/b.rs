use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(n: usize, t: usize, ct: [[usize; 2]; n]);

    let min = ct.iter().filter(|x| (*x)[1] <= t).map(|x| x[0]).min();

    println!(
        "{}",
        if min.is_some() {
            min.unwrap().to_string()
        } else {
            "TLE".to_string()
        }
    );
}
