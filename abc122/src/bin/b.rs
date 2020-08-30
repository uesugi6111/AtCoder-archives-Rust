use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input!(s: Chars);

    let sss = vec!['A', 'C', 'G', 'T'];

    let mut count = 0;
    let mut count_max = 0;
    for v in &s {
        if sss.contains(v) {
            count += 1;
        } else {
            count_max = std::cmp::max(count_max, count);
            count = 0;
        }
    }

    println!("{}", count_max);
}
