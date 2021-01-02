#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};

fn main() {
    input!(n: Chars);
    n.iter()
        .take(n.len() - 8)
        .for_each(|nn| print!("{}", nn.to_string()));
}
