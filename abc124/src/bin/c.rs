use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(s: String);
    let vs: Vec<char> = s.chars().collect();

    let mut count_even = 0;
    let mut count_odd = 0;

    for (i, cvs) in vs.iter().enumerate() {
        let is_odd = i % 2 == 0;

        if (*cvs == '0') == is_odd {
            count_even += 1;
        } else {
            count_odd += 1;
        }
    }
    let ans = if count_even < count_odd {
        count_even
    } else {
        count_odd
    };

    println!("{}", ans);
}
