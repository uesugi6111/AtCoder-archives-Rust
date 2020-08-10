#[allow(unused_imports)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(s: String);

    let mut v = s.into_bytes();
    v.sort();

    if v[0] == v[1] && v[2] == v[3] && v[0] != v[2] {
        println!("{}", "Yes");
        return;
    }

    println!("{}", "No");
}
