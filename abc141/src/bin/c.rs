#[allow(unused_imports)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(n: usize, k: isize, q: isize, a: [usize; q]);

    let mut points = vec![0 as isize; n];
    for v in a {
        points[v - 1] += 1;
    }

    for v in points {
        if v + k - q > 0 {
            println!("{}", "Yes");
        } else {
            println!("{}", "No");
        }
    }
}
