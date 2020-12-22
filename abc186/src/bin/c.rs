#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
fn main() {
    input!(n: usize);

    let mut count = 0;

    for i in 1..=n as u32 {
        if i.to_string().contains('7') {
            continue;
        }

        let str8 = format!("{:0o}", i);

        if str8.contains('7') {
            continue;
        }

        count += 1;
    }
    println!("{}", count);
}
