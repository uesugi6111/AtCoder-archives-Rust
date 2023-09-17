#[rustfmt::skip]
mod io_pro {
    #[macro_export] macro_rules! input{(sc=$sc:expr,$($r:tt)*)=>{input_inner!{$sc,$($r)*}};($($r:tt)*)=>{let mut sc=io_pro::Scanner::new(std::io::BufReader::new(std::io::stdin().lock()));input_inner!{sc,$($r)*}};}
    #[macro_export] macro_rules! input_inner{($sc:expr)=>{};($sc:expr,)=>{};($sc:expr,$var:ident:$t:tt$($r:tt)*)=>{let $var=read_value!($sc,$t);input_inner!{$sc $($r)*}};($sc:expr,mut $var:ident:$t:tt$($r:tt)*)=>{let mut $var=read_value!($sc,$t);input_inner!{$sc $($r)*}};}
    #[macro_export] macro_rules! read_value{($sc:expr,($($t:tt),*))=>{($(read_value!($sc,$t)),*)};($sc:expr,[$t:tt;$len:expr])=>{(0..$len).map(|_|read_value!($sc,$t)).collect::<Vec<_>>()};($sc:expr,Chars)=>{read_value!($sc,String).chars().collect::<Vec<char>>()};($sc:expr,Usize1)=>{read_value!($sc,usize)-1};($sc:expr,$t:ty)=>{$sc.next::<$t>()};}
    pub struct Scanner{s:Box<str>,input:std::str::SplitAsciiWhitespace<'static>,}
    impl Scanner{
        pub fn new<R:std::io::Read>(mut reader:R)->Self{let mut sc=Scanner{s:{let mut s=String::new();reader.read_to_string(&mut s).unwrap();s.into_boxed_str()},input:"".split_ascii_whitespace(),};let s:&'static str=unsafe{std::mem::transmute(&*sc.s)};sc.input=s.split_ascii_whitespace();sc}
        #[inline]pub fn next<T:std::str::FromStr>(&mut self)->T where T::Err:std::fmt::Debug,{self.input.next().unwrap().parse::<T>().expect("Parse error")}
    }
}
#[proconio::fastout]
fn main() {
    input!(t: usize, c: [i64; t]);

    for i in c {
        let mut sum = 0;
        let f = trial_division(i);

        for (k, v) in f {
            sum += k.pow(v as u32);
        }
        if sum < i {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}

use std::collections::HashMap;
pub fn trial_division(mut n: i64) -> HashMap<i64, i64> {
    let mut primes = HashMap::new();
    let mut i = 2;

    while i * i <= n {
        while n % i == 0 {
            n /= i;
            primes.entry(i).and_modify(|e| *e += 1).or_insert(1);
        }
        i += 1;
    }
    if n > 1 {
        primes.entry(n).and_modify(|e| *e += 1).or_insert(1);
    }
    primes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trial_division() {
        assert!(trial_division(25).contains_key(&5));
        assert!(trial_division(25).get(&5).unwrap() == &2);
    }
}
