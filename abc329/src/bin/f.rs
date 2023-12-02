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
use rustc_hash::{FxHashMap, FxHashSet};
#[proconio::fastout]
fn main() {
    input!(n: usize, q: usize, c: [i64; n], query: [(i64, i64); q]);

    let mut map = FxHashMap::default();

    for i in 0..n {
        map.entry(i as i64 + 1)
            .or_insert_with(FxHashSet::default)
            .insert(c[i]);
    }

    for (a, b) in query {
        //  let mut set = std::collections::HashSet::new();

        let aa = map.remove(&a).unwrap_or(FxHashSet::default());
        let bb = map.remove(&b).unwrap_or(FxHashSet::default());

        let (mut aa, bb) = if aa.len() > bb.len() {
            (aa, bb)
        } else {
            (bb, aa)
        };

        for i in bb {
            aa.insert(i);
        }
        // let cc = aa
        //     .union(&bb)
        //     .copied()
        //     .collect::<std::collections::HashSet<_>>();

        println!("{}", aa.len());

        map.insert(a, FxHashSet::default());
        map.insert(b, aa);
    }
}
