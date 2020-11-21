#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
fn main() {
    input!(_n: usize, mut s: Chars);
    let mut s_in: std::collections::VecDeque<_> = s.into_iter().collect();

    let mut ss = vec![];

    loop {
        if ss.len() < 3 {
            if s_in.is_empty() {
                break;
            }
            ss.push(s_in.pop_front());
            continue;
        }
        let mut str = "".to_string();
        str.push(ss[ss.len() - 3].unwrap());
        str.push(ss[ss.len() - 2].unwrap());
        str.push(ss[ss.len() - 1].unwrap());

        if str == "fox" {
            ss.pop();
            ss.pop();
            ss.pop();
        }
        if s_in.is_empty() {
            break;
        }
        ss.push(s_in.pop_front());
    }

    println!("{}", ss.len());
}
