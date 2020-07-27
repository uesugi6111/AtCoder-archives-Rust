#[allow(unused_imports)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(n: usize, m: usize, ps: [(usize, String); m]);

    let mut wa: isize = 0;
    let mut ac: isize = 0;
    let mut bool_array: Vec<isize> = vec![0; n];
    for v in ps {
        if bool_array[v.0 - 1] != -1 {
            if v.1 == "AC" {
                wa += bool_array[v.0 - 1];
                bool_array[v.0 - 1] = -1;
                ac += 1;
            } else {
                bool_array[v.0 - 1] += 1;
            }
        }
    }

    println!("{} {}", ac, wa);
}
