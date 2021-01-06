#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
fn main() {
    input!(n: usize, s: [String; n]);

    let ex = "!".to_string();

    let mut set = std::collections::HashSet::new();

    for ss in s {
        let buff = if ss.starts_with('!') {
            let mut sss = ss.clone();
            sss.remove(0);
            sss
        } else {
            format!("{}{}", ex, ss)
        };
        if set.contains(&buff) {
            let ssss = if buff.starts_with('!') {
                let mut sss = buff;
                sss.remove(0);
                sss
            } else {
                buff
            };

            println!("{}", ssss);
            return;
        } else {
            set.insert(ss);
        }
    }

    println!("satisfiable");
}
