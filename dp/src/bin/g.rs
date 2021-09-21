#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
use std::collections::HashMap;
#[fastout]
fn main() {
    input!(n: usize, m: usize, xy: [(i64, i64); m]);

    let mut map = HashMap::new();

    for (x, y) in xy {
        map.entry(x).or_insert(vec![]).push(y);
    }
    let mut memo = HashMap::new();

    let mut ans = 0;
    for i in 1..=n {
        ans = std::cmp::max(ans, excute(i as i64, &map, &mut memo));
    }

    println!("{}", ans);
}

fn excute(x: i64, map: &HashMap<i64, Vec<i64>>, memo: &mut HashMap<i64, i64>) -> i64 {
    match map.get(&x) {
        Some(v) => {
            v.iter()
                .map(|y| -> i64 {
                    match memo.get(y) {
                        Some(ans) => *ans,
                        None => {
                            let ans = excute(*y, map, memo);
                            memo.insert(*y, ans);
                            ans
                        }
                    }
                })
                .max()
                .unwrap()
                + 1
        }
        None => 0,
    }
}
