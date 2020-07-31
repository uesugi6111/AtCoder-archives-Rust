#[allow(unused_imports)]
use proconio::{fastout, input};

#[macro_use]
extern crate lazy_static;

use std::collections::HashMap;

lazy_static! {
    static ref HASHMAP: HashMap<&'static str, u32> = [
        ("SUN", 7),
        ("MON", 6),
        ("TUE", 5),
        ("WED", 4),
        ("THU", 3),
        ("FRI", 2),
        ("SAT", 1),
    ]
    .iter()
    .copied()
    .collect();
}

#[fastout]
fn main() {
    input!(s: String);

    println!("{}", HASHMAP.get::<str>(&s.to_string()).unwrap());
}
