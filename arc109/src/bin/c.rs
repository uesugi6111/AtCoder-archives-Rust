#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
fn main() {
    input!(n: i64, k: i64, s: Chars);

    let mut ss = vec![];

    let mut kk = 2;

    let mut k_count = 1;
    while k_count < std::cmp::min(k, 27) {
        kk *= 2;
        k_count += 1;
    }

    for i in 0..kk {
        ss.push(s[(i % n) as usize]);
    }

    let mut v = ss;

    while v.len() != 1 {
        let mut buff = vec![];
        for i in 0..v.len() / 2 {
            buff.push(get_win_char(v[2 * i], v[2 * i + 1]));
        }

        v = buff;
    }

    println!("{}", v[0]);
}

fn get_win_char(c1: char, c2: char) -> char {
    if c1 == 'R' {
        if c2 == 'S' {
            'R'
        } else if c2 == 'P' {
            'P'
        } else {
            'R'
        }
    } else if c1 == 'S' {
        if c2 == 'P' {
            'S'
        } else if c2 == 'R' {
            'R'
        } else {
            'S'
        }
    } else {
        if c2 == 'S' {
            'S'
        } else if c2 == 'R' {
            'P'
        } else {
            'P'
        }
    }
}
