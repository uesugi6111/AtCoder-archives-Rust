#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
fn main() {
    input!(
        h: usize,
        w: usize,
        n: usize,
        m: usize,
        ab: [(usize, usize); n],
        cd: [(usize, usize); m]
    );

    let mut h_map = std::collections::HashMap::new();
    let mut w_map = std::collections::HashMap::new();

    for v in cd {
        h_map.entry(v.0).or_insert(vec![]).push(v.1);
        w_map.entry(v.1).or_insert(vec![]).push(v.0);
    }
    // for (_, m) in h_map.i {
    //     m.sort();
    // }

    println!("{}", n);
}
