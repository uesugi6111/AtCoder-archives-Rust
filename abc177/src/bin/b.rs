use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(s: String, t: String);
    let vs: Vec<char> = s.chars().collect();
    let vt: Vec<char> = t.chars().collect();

    let mut count_min = vt.len();

    for i in 0..(vs.len() - vt.len() + 1) {
        let mut count = 0;
        for j in 0..vt.len() {
            if vs[i + j] != vt[j] {
                count += 1;
            }
        }
        if count < count_min {
            count_min = count;
        }
    }
    println!("{}", count_min);
}
