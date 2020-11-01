use itertools::Itertools;
#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
fn main() {
    input!(s: Chars);

    if s.len() == 1 {
        let input: i32 = s[0] as i32 - 48;
        if input % 8 == 0 {
            println!("Yes");
            return;
        }
    } else if s.len() == 2 {
        let inputl: i32 = s[0] as i32 - 48;
        let inputr: i32 = s[1] as i32 - 48;

        if (inputl * 10 + inputr) % 8 == 0 || (inputl + inputr * 10) % 8 == 0 {
            println!("Yes");
            return;
        }
    } else {
        // 最大3つの配列にする
        let mut ss = vec![];
        let mut count = vec![0; 10];
        for i in &s {
            let buff = *i as i32 - 48;
            if count[buff as usize] < 3 {
                ss.push(buff);
                count[buff as usize] += 1;
            }
        }

        for c in ss.iter().permutations(3) {
            if (*c[0] * 100 + *c[1] * 10 + *c[2]) % 8 == 0 {
                println!("Yes");
                return;
            }
        }
    }

    println!("No");
}
