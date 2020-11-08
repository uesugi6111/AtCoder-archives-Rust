#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
fn main() {
    input!(n: usize,mut a:[usize;n]);

    let max = a.iter().max().unwrap();

    let mut ans_max = 0;
    let mut ind = 0;

    for i in 2..*max + 1 {
        let mut count = 0;
        for j in &a {
            if j % i == 0 {
                count += 1;
            }
        }

        if ans_max <= count {
            ind = i;
            ans_max = std::cmp::max(ans_max, count);
        }
    }
    println!("{}", ind);
}
