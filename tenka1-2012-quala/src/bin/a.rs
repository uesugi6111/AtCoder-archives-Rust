#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
fn main() {
    input!(n: usize);

    let mut q = std::collections::VecDeque::new();

    q.push_back(0);
    q.push_back(1);
    let mut sum: i64 = 1;
    for _ in 0..n {
        let a = q.pop_front().unwrap();

        q.push_back(sum + a);
        sum += a;
    }

    println!("{}", sum);
}
