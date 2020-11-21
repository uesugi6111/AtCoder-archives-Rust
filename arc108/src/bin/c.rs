#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
fn main() {
    input!(n: usize, m: usize, uvc: [(usize, usize, usize); m]);

    uvc.iter().filter(|x|->bool{
        if x.0 ==x.1{
            return false;
        }
        
    })

    println!("{}", n);
}
