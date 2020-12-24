#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
fn main() {
    input!(n: i64, m: usize);
    if m == 0 {
        println!("1");
        return;
    }

    input!( mut a: [i64; m]);
    a.sort();
    a.push(n + 1);

    let mut buff = 0;
    let v: Vec<_> = a
        .iter()
        .filter_map(|x| -> Option<_> {
            let aaa = x - buff - 1;
            buff = *x;
            if aaa <= 0 {
                None
            } else {
                Some(aaa)
            }
        })
        .collect();

    let min = v.iter().min().unwrap_or(&0);

    let sum: i64 = v
        .iter()
        .map(|x| x / min + if x % min == 0 { 0 } else { 1 })
        .sum();

    println!("{}", sum);
}
