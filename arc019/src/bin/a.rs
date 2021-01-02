#[allow(unused_imports)]
use proconio::{
    input,
    marker::{Bytes, Chars, Isize1, Usize1},
};

fn main() {
    input!(s: Chars);

    s.iter()
        .map(|x| match x {
            'O' => '0',
            'D' => '0',
            'I' => '1',
            'Z' => '2',
            'S' => '5',
            'B' => '8',
            _ => *x,
        })
        .for_each(|x| print!("{}", x));
    println!("");
}
