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
use std::time::Instant;
#[proconio::fastout]
fn main() {
    input!(n: usize, x: i64, mut ab: [(i64, i64); n]);

    let mut time = 0;
    let since = Instant::now();

    ab.sort_by_key(|&x| -x.0);

    let ba = ab.iter().rev().map(|&x| x.0 * x.1).collect::<Vec<_>>();
    let cumsum = cumsum(&ba).iter().copied().rev().collect::<Vec<_>>();

    let ans = f(&ab, &cumsum, 0, x, 0, time, since);

    println!("{}", if ans { "Yes" } else { "No" });
}

fn f(
    ab: &[(i64, i64)],
    csum: &[i64],
    index: usize,
    x: i64,
    now: i64,
    time: i64,
    since: Instant,
) -> bool {
    if Instant::now().duration_since(since).as_millis() > 2800 {
        return false;
    }
    if ab.len() == index {
        if x == now {
            return true;
        } else {
            return false;
        }
    }
    if now + csum[index] < x {
        return false;
    }

    for i in 0..ab[index].1 + 1 {
        if now + ab[index].0 * i > x {
            return false;
        }
        if f(ab, csum, index + 1, x, now + ab[index].0 * i, time, since) {
            return true;
        }
    }
    false
}

pub fn cumsum(v: &[i64]) -> Vec<i64> {
    (0..1)
        .chain(v.iter().scan(0, |c, &x| {
            *c += x;
            Some(*c)
        }))
        .collect()
}
