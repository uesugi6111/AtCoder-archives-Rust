use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input!(s: Chars);

    let mut count_0 = 0;
    let mut count_1 = 0;

    for v in &s {
        if *v == '0' {
            count_0 += 1;
        } else {
            count_1 += 1;
        }
    }

    println!("{}", std::cmp::min(count_0, count_1) * 2);
}
