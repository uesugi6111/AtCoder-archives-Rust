use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(s: String);
    let ss: Vec<usize> = (&s).split(&"/").map(|x| x.parse().unwrap()).collect();

    let ans = if ss[0] <= 2019 && ss[1] <= 4 && ss[2] <= 30 {
        "Heisei"
    } else {
        "TBD"
    };

    println!("{}", ans);
}
