use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(x: usize, y: usize);
    if x % y == 0 {
        println!("-1");
        return;
    }

    let mut buff = x;
    while buff < 1_000_000_000_000_000_000 {
        buff += x;
        if buff % y != 0 {
            println!("{}", buff);
            return;
        }
    }
    println!("-1");
}
