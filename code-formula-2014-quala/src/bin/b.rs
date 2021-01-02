#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
fn main() {
    input!(a: usize, b: usize, p: [char; a], q: [char; b]);

    let mut vvv = vec![];
    vvv.push(vec![7, 8, 9, 0]);
    vvv.push(vec![4, 5, 6]);
    vvv.push(vec![2, 3]);
    vvv.push(vec![1]);

    for i in 0..4 {
        for _ in 0..i {
            print!(" ");
        }

        for u in &vvv[i] {
            let c = std::char::from_digit(*u, 10).unwrap();
            print!(
                "{} ",
                if p.contains(&c) {
                    '.'
                } else if q.contains(&c) {
                    'o'
                } else {
                    'x'
                }
            );
        }
        println!();
    }
}
