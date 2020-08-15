#[allow(unused_imports)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(s: String, t: String);
    let v_s: Vec<_> = s.chars().collect();

    let v_t: Vec<_> = t.chars().collect();
    let mut count = 0;
    for i in 0..v_s.len() {
        if v_s[i] != v_t[i] {
            count += 1;
        }
    }

    println!("{}", count);
}
