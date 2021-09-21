#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
fn main() {
    input!(n: usize);
    println!("{}", n);
}
#[test]
fn aa() {
    let mut test = std::collections::HashMap::new();

    let data = vec![
        ("aaa", "1111"),
        ("aaa", "2222"),
        ("bbb", "3333"),
        ("ccc", "4444"),
    ];

    for (key, value) in data {
        test.entry(key).or_insert_with(|| vec![]).push(value);
    }

    for (key, value) in test.iter() {
        for v in value {
            println!("key={},value={}", &key, &v);
        }
    }
}
