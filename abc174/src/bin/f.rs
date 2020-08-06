#[allow(unused_imports)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(n: usize, q: usize, _c: [usize; n], lr: [[usize; 2]; q]);

    let mut lr_vec = vec![vec![0; 2]; q];
    for i in 0..lr.len() {
        lr_vec[(lr[i][0] - 1) as usize] = vec![lr[i][1] - 1, i];
    }

    lr_vec.sort();
    println!("{:?}", lr_vec);
}
