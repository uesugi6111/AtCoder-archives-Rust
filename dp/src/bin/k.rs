use std::vec;

#[rustfmt::skip]
mod fast_input {
    #[macro_export]macro_rules! input{($($r:tt)*)=>{let s={use std::io::Read;let mut s=String::new();std::io::stdin().read_to_string(&mut s).unwrap();let input=Box::leak(s.into_boxed_str());input};let mut iter=s.split_ascii_whitespace();input_inner!{iter,$($r)*}};}
    #[macro_export]macro_rules! input_inner{($iter:expr)=>{};($iter:expr,)=>{};($iter:expr,$var:ident:$t:tt$($r:tt)*)=>{let $var=read_value!($iter,$t);input_inner!{$iter $($r)*}};}
    #[macro_export]macro_rules! read_value{($iter:expr,($($t:tt),*))=>{($(read_value!($iter,$t)),*)};($iter:expr,[$t:tt;$len:expr])=>{(0..$len).map(|_|read_value!($iter,$t)).collect::<Vec<_>>()};($iter:expr,Chars)=>{read_value!($iter,String).chars().collect::<Vec<char>>()};($iter:expr,Usize1)=>{read_value!($iter,usize)-1};($iter:expr,$t:ty)=>{$iter.next().unwrap().parse::<$t>().expect("Parse error")};}
}
#[proconio::fastout]
fn main() {
    input!(n: usize, k: usize, a: [i64; n]);

    let mut v = vec![false; k + 1];

    for i in 1..k + 1 {
        if v[k - i] {
            continue;
        }
        for j in &a {
            if k - i <= *j as usize {
                continue;
            }
            v[k - i - 1 - *j as usize] = true;
        }
    }

    println!("{}", if v[0] { "First" } else { "Second" });
}
