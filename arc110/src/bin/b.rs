#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
fn main() {
    input!(n: i64, t: String);

    let base = vec!['1', '1', '0'];

    let mut count = 0;
    for i in 0..3 {
        let mut flag = true;
        for (j, c) in t.as_str().chars().enumerate() {
            if c != base[(j + i) % 3] {
                flag = false;
                break;
            }
        }
        if flag {
            count += 1;
        }
    }
    if count == 0 {
        println!("0");
        return;
    }

    println!(
        "{}",
        10_000_000_000i64 * count - if 3 >= n { 0 } else { (n) / 3 + 1 }
    );
}
