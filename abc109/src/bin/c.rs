#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
fn main() {
    input!(n: usize, x: isize,mut  nx: [isize; n]);

    nx.sort();

    let mut nnx = vec![0; n];

    for i in 0..nx.len() {
        nnx[i] = (x - nx[i]).abs();
    }

    nnx.sort();

    let mut ans = 1;

    for i in (2..=nnx[0]).rev() {
        let mut is_divdiv = true;
        for v in &nnx {
            if *v % i != 0 {
                is_divdiv = false;
                break;
            }
        }
        if is_divdiv {
            ans = i;
            break;
        }
    }

    println!("{}", ans);
}
