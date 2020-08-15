#[allow(unused_imports)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(s: String);

    let mut count = 0;
    let mut max = 0;
    for v in s.chars() {
        if v == 'R' {
            count += 1;

            max = if max < count { count } else { max };
        } else {
            count = 0;
        }
    }

    println!("{}", max);
}
