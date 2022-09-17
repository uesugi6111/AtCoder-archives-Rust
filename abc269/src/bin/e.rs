use std::io::{self, BufReader};

use proconio::{input, source::line::LineSource};

fn main() {
    let mut stdin = LineSource::new(BufReader::new(io::stdin()));
    macro_rules! input(($($tt:tt)*) => (proconio::input!(from &mut stdin, $($tt)*)));

    input!(n: usize);

    let mut h_range = 1..=n;
    while h_range.end() != h_range.start() {
        let buff = *h_range.start() + (*h_range.end() - *h_range.start()) / 2;
        println!("? {} {} {} {}", *h_range.start(), buff, 1, n);
        input!(x: usize);
        if buff - h_range.start() + 1 == x {
            h_range = buff + 1..=*h_range.end();
        } else {
            h_range = *h_range.start()..=buff;
        }
    }

    let mut w_range = 1..=n;
    while w_range.end() != w_range.start() {
        let buff = *w_range.start() + (*w_range.end() - *w_range.start()) / 2;
        println!("? {} {} {} {}", 1, n, w_range.start(), buff,);
        input!(x: usize);
        if buff - w_range.start() + 1 == x {
            w_range = buff + 1..=*w_range.end();
        } else {
            w_range = *w_range.start()..=buff;
        }
    }
    println!("! {} {}", h_range.start(), w_range.start());
}
