#[proconio::fastout]
fn main() {
    proconio::input!(b: u128);

    let mut buff = 1u128;
    let mut bb = 1;
    while bb <= b {
        bb = buff.pow(buff as u32);

        if bb == b {
            println!("{}", buff);
            return;
        }
        buff += 1;
    }

    println!("-1");
}
