use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(n: usize, m: usize);

    let mut m_v = vec![0usize; m];

    for _i in 0..n {
        input!(k: usize, ka: [usize; k]);
        for v in &ka {
            m_v[*v - 1] += 1;
        }
    }
    println!("{}", m_v.iter().filter(|&x| *x == n).count());
}
