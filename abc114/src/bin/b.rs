use proconio::*;

#[fastout]
fn main() {
    input!(s: String);
    let ss: String = s;

    let mut min = 1_000;
    for i in 0..ss.len() - 2 {
        min = std::cmp::min(min, (753 - &ss[i..3 + i].parse::<i32>().unwrap()).abs());
    }

    println!("{}", min);
}
