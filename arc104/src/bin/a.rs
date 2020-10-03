#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
fn main() {
    input!(a: isize,b:isize);


    for i in 0..401{
        for j in 0..401{
            if (i-200)+(j-200) == a&&(i-200)-(j-200) == b{

                println!("{} {}",(i-200),(j-200)); return ;
            }
        }
    }






}
