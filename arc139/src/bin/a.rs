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
    input!(n: usize, t: [u32; n]);

    let mut max = 1_u64 << t[0];

    for i in 1..n {
        if t[i - 1] > t[i] {
            max += 1_u64 << t[i];
        } else if t[i - 1] == t[i] {
            max += 1_u64 << (t[i] + 1);
        } else {
            max = (max & (std::u64::MAX << t[i])) + (1_u64 << t[i]);
            if max.trailing_zeros() != t[i] {
                max += (1_u64 << t[i]);
            }
        }
        // let buff = 1_u64 << t[i];
        // if buff > max {
        //     max = buff;
        // } else if t[i] == 0 {
        //     if max.trailing_zeros() == 0 {
        //         max += 2;
        //     } else {
        //         max += 1;
        //     }
        // } else {
        //     max = (max & (std::u64::MAX << t[i])) + (1_u64 << t[i]);
        // }
    }
    println!("{}", max);
}

#[test]
fn a() {
    println!("{}", 4i64.trailing_zeros());
}
