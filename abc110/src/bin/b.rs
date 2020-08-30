use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(n: usize,m:usize,x:isize,y:isize,mut xn:[isize;n],mut ym:[isize;m]);

    xn.sort();
    ym.sort();

    let is_oc = if xn[xn.len() - 1] >= ym[0] {
        true
    } else {
        let count = (xn[xn.len() - 1] + 1..ym[0] + 1)
            .filter(|&xxx| x < xxx && xxx <= y)
            .count();
        count == 0
    };

    println!("{}", if is_oc { "War" } else { "No War" });
}
