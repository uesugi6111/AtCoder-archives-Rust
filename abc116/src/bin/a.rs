use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(n: [usize; 3]);

    let max = n.iter().max().unwrap();

    let mut menseki: f64 = 1.0;
    for v in &n {
        if *v == *max {
            continue;
        }
        menseki *= *v as f64;
    }

    println!("{}", menseki / 2.0);
}
