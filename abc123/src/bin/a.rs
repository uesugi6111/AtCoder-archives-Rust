use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(n: [isize; 5], k: usize);

    let mut is_ = false;
    for v in &n {
        for v2 in &n {
            if (v - v2).abs() as usize > k {
                is_ = true;
            }
        }
    }

    println!("{}", if is_ { ":(" } else { "Yay!" });
}
