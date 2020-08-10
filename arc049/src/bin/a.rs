#[allow(unused_imports)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(s: String, a: [usize; 4]);
    let vs: Vec<_> = s.chars().collect();
    for i in 0..vs.len() {
        if a.contains(&i) {
            print!("{}", "\"");
        }
        print!("{}", vs[i]);
    }
    if a.contains(&vs.len()) {
        print!("{}", "\"");
    }
    print!("{}", "\n");
}
