#[allow(unused_imports)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(s: String, t: String);

    let s_v: Vec<char> = s.chars().collect();
    let t_v: Vec<char> = t.chars().collect();
    let mut count = 0;
    for i in 0..s.len() {
        if s_v[i] == t_v[i] {
            count += 1;
        }
    }
    println!("{}", count);
}
