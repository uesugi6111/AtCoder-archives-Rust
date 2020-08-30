use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(n: usize,m:usize,mut ab:[[usize;2];n]);
    let mut aaa = Vec::new();
    ab.sort();
    for v in &ab {
        for _vv in 0..v[1] {
            aaa.push(v[0]);
            if aaa.len() > m {
                break;
            }
        }
    }
    aaa.sort();

    println!("{}", aaa.iter().take(m).sum::<usize>());
}
