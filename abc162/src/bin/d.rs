#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
fn main() {
    input!(_n: usize, s: Chars);

    let mut count_r = 0;
    let mut count_g = 0;
    let mut count_b = 0;

    for v in &s {
        match v {
            'R' => count_r += 1,
            'G' => count_g += 1,
            'B' => count_b += 1,
            _ => (),
        }
    }
    let mut sum: usize = count_r * count_g * count_b;

    for i in 0..s.len() - 2 {
        for j in i + 1..s.len() - 1 {
            if s[i] == s[j] {
                continue;
            }
            let ji = j - i;
            if j + ji > s.len() - 1 {
                continue;
            }

            if s[i] != s[j + ji] && s[j] != s[j + ji] {
                sum -= 1;
            }
        }
    }

    println!("{}", sum);
}
