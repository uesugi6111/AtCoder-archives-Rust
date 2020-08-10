#[allow(unused_imports)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(_n: usize);

    let mut aa = 1;
    for _i in 0..1000 {
        aa = (aa * 2) % 1000_000_007;
        if aa == 608200469 {
            println!("{}", "Yes\n\n");
        }
        println!("{}", aa);
    }
}
