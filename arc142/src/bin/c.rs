use std::io::{self, BufReader};

use proconio::{input, source::line::LineSource};
fn main() {
    let mut source = LineSource::new(BufReader::new(io::stdin()));
    input! (        from &mut source,        n: usize    );

    let mut v = vec![0; n - 2];
    for i in 1..=2 {
        for j in 0..n - 2 {
            println!("? {} {}", i, j + 3);
            input!(   from &mut source,d: i64);
            v[j] += d;
        }
    }

    let mut map = std::collections::BTreeMap::new();
    for i in 0..n - 2 {
        map.entry(v[i]).or_insert_with(Vec::new).push(i + 3);
    }
    let a = map.iter().next().unwrap();
    if *a.0 > 3
    //&& *a.0 == a.1.len() as i64 + 1
    {
        println!("! {}", a.0);
        return;
    }
    if n == 3 && *a.0 == 3 {
        println!("! 1");
        return;
    }
    if *a.0 == 3 && a.1.len() == 2 {
        println!("? {} {}", a.1[0], a.1[1]);
        input!(   from &mut source,d: i64);
        if d == 1 {
            println!("! 3");
        } else {
            println!("! 1");
        }
    } else if *a.0 == 2 && a.1.len() == 1 {
        println!("! 2");
    } else {
        println!("! 1");
    }
}
