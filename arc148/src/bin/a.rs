use itertools::Itertools;

#[rustfmt::skip]
mod io_pro {
    #[macro_export] macro_rules! input{(sc=$sc:expr,$($r:tt)*)=>{input_inner!{$sc,$($r)*}};($($r:tt)*)=>{let mut sc=io_pro::Scanner::new(std::io::stdin().lock());input_inner!{sc,$($r)*}};}
    #[macro_export] macro_rules! input_inner{($sc:expr)=>{};($sc:expr,)=>{};($sc:expr,$var:ident:$t:tt$($r:tt)*)=>{let $var=read_value!($sc,$t);input_inner!{$sc $($r)*}};($sc:expr,mut $var:ident:$t:tt$($r:tt)*)=>{let mut $var=read_value!($sc,$t);input_inner!{$sc $($r)*}};}
    #[macro_export] macro_rules! read_value{($sc:expr,($($t:tt),*))=>{($(read_value!($sc,$t)),*)};($sc:expr,[$t:tt;$len:expr])=>{(0..$len).map(|_|read_value!($sc,$t)).collect::<Vec<_>>()};($sc:expr,Chars)=>{read_value!($sc,String).chars().collect::<Vec<char>>()};($sc:expr,Usize1)=>{read_value!($sc,usize)-1};($sc:expr,$t:ty)=>{$sc.next::<$t>()};}
    pub struct Scanner{s:Box<str>,input:std::iter::Peekable<std::str::SplitAsciiWhitespace<'static>>,}
    impl Scanner{
        pub fn new<R:std::io::Read>(mut reader:R)->Self{let mut sc=Scanner{s:{let mut s=String::new();reader.read_to_string(&mut s).unwrap();s.into_boxed_str()},input:"".split_ascii_whitespace().peekable(),};let s:&'static str=unsafe{std::mem::transmute(&*sc.s)};sc.input=s.split_ascii_whitespace().peekable();sc}
        #[inline]pub fn next<T:std::str::FromStr>(&mut self)->T where T::Err:std::fmt::Debug,{self.input.next().unwrap().parse::<T>().expect("Parse error")}
    }
}
#[proconio::fastout]
fn main() {
    input!(n: usize,mut a:[i64;n]);
    a.sort_unstable();

    let mmm = (1..n)
        .map(|i| a[i] - a[i - 1])
        .filter(|&x| x > 2)
        .sorted()
        .collect::<Vec<_>>();

    let m = (1..n).map(|i| a[i] - a[i - 1]).filter(|&x| x > 2).max();

    // let mut m = 0;
    // for i in 1..n {
    //     if a[i] - a[i - 1] > 2 {
    //         m = a[i] - a[i - 1];
    //         break;
    //     }
    // }

    if let Some(mm) = m {
        let hh = enum_divisors(mm);
        for h in hh {
            if h <= 2 {
                continue;
            }
            let mut set = std::collections::HashSet::new();
            for i in 0..n {
                set.insert(a[i] % h);
                if set.len() > 1 {
                    break;
                }
            }
            if set.len() == 1 {
                println!("1");
                return;
            }
        }
    }

    for j in 1..n {
        if a[0] % 2 != a[j] % 2 {
            println!("2");
            return;
        }
    }
    println!("1");
    return;
}

pub fn enum_divisors(n: i64) -> Vec<i64> {
    let mut res = vec![];
    for i in 1..=(n as f64).sqrt() as i64 {
        if n % i != 0 {
            continue;
        }
        res.push(i);
        if i.pow(2) != n {
            res.push(n / i);
        }
    }
    res
}
