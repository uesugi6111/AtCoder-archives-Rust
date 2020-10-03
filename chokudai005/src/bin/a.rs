#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
fn main() {
    input!(_id:usize,n: usize,_k:usize,_s:[Chars;n]);



    let out=std::cmp::min(10000,n*n);
    println!("{}", out);
    
    let mut count = 0; 
    for i in 0..n{
        for j in 0..n{
            println!("{} {} 1",i,j);
            count+=1;
            if count >=out{
return ;
            }
        }
    }
}
 