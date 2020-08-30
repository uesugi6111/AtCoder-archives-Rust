use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(s: usize);

    let mut buff: usize = s;
    let mut set = std::collections::HashSet::new();
    set.insert(buff);

    let mut count = 1;
    loop {
        if buff % 2 == 0 {
            buff /= 2;
        } else {
            buff = 3 * buff + 1;
        }
        count += 1;
        if set.contains(&buff) {
            break;
        }
        set.insert(buff);
    }

    println!("{}", count);
}
