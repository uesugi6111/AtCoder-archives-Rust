#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
fn main() {
    input!(n: usize, a: [isize; n]);

    let mut ruisekiwa: Vec<isize> = vec![];
    ruisekiwa.push(a[0]);

    for (i, j) in a.iter().enumerate().skip(1) {
        ruisekiwa.push(ruisekiwa[i - 1] + j);
    }

    let mut max = 0;
    let mut sum: isize = 0;

    let mut high = 0;

    for i in ruisekiwa {
        high = std::cmp::max(high, i);
        if high <= i {
            sum += i;
            max = std::cmp::max(max, sum);
        } else {
            max = std::cmp::max(max, sum + high);
            sum += i;
        }
    }

    println!("{}", max);
}
