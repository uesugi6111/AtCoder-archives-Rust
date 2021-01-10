#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
fn main() {
    input!(n: usize, c: i64, abc: [(usize, usize, i64); n]);

    let mut tree_map: std::collections::BTreeMap<usize, i64> = std::collections::BTreeMap::new();

    for (a, b, c) in abc {
        *tree_map.entry(a).or_insert(0) += c;
        *tree_map.entry(b + 1).or_insert(0) -= c;
    }

    let mut buff = (0, 0);
    let mut cumcum = 0;
    let mut sum = 0;
    for i in tree_map {
        let index_sub = (i.0 - buff.0) as i64;
        sum += std::cmp::min(cumcum, c) * index_sub;
        cumcum += i.1;

        buff = i;
    }

    println!("{}", sum);
}
