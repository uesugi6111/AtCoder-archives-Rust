#[allow(unused_imports)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(s: String);

    let vs: Vec<char> = s.chars().collect();

    let s_f: usize = format!("{}{}", vs[0], vs[1]).parse().unwrap();
    let s_b: usize = format!("{}{}", vs[2], vs[3]).parse().unwrap();

    let is_month_b = !(s_b > 12 || s_b == 0);
    let is_month_f = !(s_f > 12 || s_f == 0);

    let ans = if is_month_b && is_month_f {
        "AMBIGUOUS"
    } else if is_month_b {
        "YYMM"
    } else if is_month_f {
        "MMYY"
    } else {
        "NA"
    };

    println!("{}", ans);
}
