#[allow(unused_imports)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(n: usize, s: String, t: String);

    let ss: Vec<char> = s.chars().collect();
    let tt: Vec<char> = t.chars().collect();

    let mut st: String = "".to_string();
    for i in 0..n {
        st.push(ss[i]);
        st.push(tt[i]);
    }

    println!("{}", st);
}
