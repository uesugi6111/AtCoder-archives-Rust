#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
fn main() {
    input!(n: usize, xy: [(isize, isize); n]);

    let x_max = xy.iter().map(|x| x.0).max().unwrap();
    let x_min = xy.iter().map(|x| x.0).min().unwrap();
    let y_max = xy.iter().map(|x| x.1).max().unwrap();
    let y_min = xy.iter().map(|x| x.1).min().unwrap();

    let x: Vec<(isize, isize)> = xy
        .into_iter()
        .filter(|x| x.0 == x_max || x.0 == x_min || x.1 == y_max || x.1 == y_min)
        .collect();

    let mut ans = 0;
    for i in 0..x.len() - 1 {
        for j in i + 1..x.len() {
            ans = std::cmp::max(ans, (x[i].0 - x[j].0).abs() + (x[i].1 - x[j].1).abs());
        }
    }

    println!("{}", ans);
}
