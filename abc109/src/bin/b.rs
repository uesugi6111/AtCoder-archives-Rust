#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};

#[fastout]
fn main() {
    input!(n: usize, w: [Chars; n]);

    for i in 0..w.len() - 1 {
        if w[i][w[i].len() - 1] != w[i + 1][0] {
            println!("No");
            return;
        }
    }

    let ss: std::collections::HashSet<String> = w.iter().map(|x| x.into_iter().collect()).collect();
    if w.len() != ss.len() {
        println!("No");
        return;
    }
    println!("Yes");
}
