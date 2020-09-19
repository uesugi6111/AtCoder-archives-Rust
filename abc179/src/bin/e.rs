#![allow(dead_code)]
#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
fn main() {
    input!(n: usize, x: usize, m: usize);

    let mut ans = Vec::with_capacity(1000);
    ans.push(x);
    let mut set = std::collections::HashSet::new();

    for i in 1..n {
        let buff = (ans[i - 1] * ans[i - 1]) % m;
        if set.contains(&buff) {
            let pos = ans.iter().position(|x| *x == buff).unwrap();
            let ans_len = ans.len();
            let wa = ans_len - pos;

            let count = (n - ans_len) / wa;
            let count_m = (n - ans_len) % wa;

            let mut sum = ans.iter().take(ans_len - wa + count_m).sum::<usize>();

            sum += ans.iter().skip(ans_len - wa).sum::<usize>() * (count + 1);
            println!("{}", sum);
            return;
        }
        set.insert(buff);
        ans.push(buff);
    }

    println!("{}", ans.iter().sum::<usize>());
}
