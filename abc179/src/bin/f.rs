#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
fn main() {
    input!(n: usize, q: usize, query: [(usize, usize); q]);

    let mut min_xv = vec![n - 2; n - 2];
    let mut min_yv = vec![n - 2; n - 2];
    let mut min_x = n - 2;
    let mut min_y = n - 2;
    let mut ans = (n - 2).pow(2);

    for que in query {
        let q1 = que.1 - 2;
        if que.0 == 1 {
            if min_x > q1 {
                for i in q1..min_x {
                    min_xv[i] = min_y;
                }
                min_x = q1;
            }
            ans -= min_xv[q1];
        } else {
            if min_y > q1 {
                for i in q1..min_y {
                    min_yv[i] = min_x;
                }
                min_y = q1;
            }
            ans -= min_yv[q1];
        }
    }

    println!("{}", ans);
}
