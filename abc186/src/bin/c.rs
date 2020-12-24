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

        let mut is_nana = false;
        for j in 0..10 as u32 {
            let bit = 7 << (j * 3);
            if i & bit == bit {
                is_nana = true;
                break;
            }
        }
        if is_nana {
            continue;
        }

        count += 1;
    }
    println!("{}", count);
}
