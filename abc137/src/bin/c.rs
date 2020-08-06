#[allow(unused_imports)]
use proconio::{fastout, input};
use std::collections::HashMap;

#[fastout]
fn main() {
    input!(n: usize, mut s: [String; n]);
    s.sort_unstable();

    let mut sss: Vec<String> = Vec::new();

    for v in s {
        let mut a: Vec<char> = v.chars().collect::<Vec<char>>();
        a.sort_unstable();
        sss.push(a.into_iter().collect());
    }

    sss.sort_unstable();

    let mut count = 0;
    let mut map = HashMap::new();
    for i in 0..sss.len() {
        let buff = map.get(&sss[i]);
        if buff == None {
            map.insert(&sss[i], 1 as usize);
        } else {
            let a = buff.unwrap() + 1;
            map.insert(&sss[i], a);
        }
    }

    for (_k, v) in map {
        count += (v * (v - 1)) / 2;
    }

    println!("{}", count);
}
