#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
fn main() {
    input!(m: usize, b: [usize; m]);
    let mut ans = vec![0usize; m + 1];

    for (i, v) in b.iter().enumerate().rev() {
        let mut a = 0;

        for j in 2..m + 1 {
            if (i + 1) * j > m {
                break;
            }
            a += ans[(i + 1) * j];
        }

        if a % 2 != *v {
            ans[(i + 1)] = 1;
        }
    }
    let aaans: Vec<usize> = ans
        .iter()
        .enumerate()
        .filter(|x| *x.1 == 1)
        .map(|(x, _v)| x)
        .collect();
    if aaans.is_empty() {
        println!("0");
    } else {
        println!("{}", aaans.len());
        for i in aaans {
            print!("{} ", i);
        }
    }
}
