#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
fn main() {
    input!(n: usize, k: usize, a: [i64; n]);

    let mut v = vec![0; n + 1];

    for aa in a {
        v[aa as usize] += 1;
    }

    let mut count = 0;
    let mut ans = 0;
    while v[0] > 0 && count < k {
        let mut buff = 0;
        for i in 0..=n + 1 {
            if v[i] == 0 {
                buff = i;
                break;
            }
            v[i] -= 1;
        }
        ans += buff;
        count += 1;
    }

    println!("{}", ans);
}
