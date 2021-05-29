#[rustfmt::skip]
mod io_pro {
    #[macro_export] macro_rules! input{(sc=$sc:expr,$($r:tt)*)=>{input_inner!{$sc,$($r)*}};($($r:tt)*)=>{let mut sc=io_pro::Scanner::new(std::io::stdin().lock());input_inner!{sc,$($r)*}};}
    #[macro_export] macro_rules! input_inner{($sc:expr)=>{};($sc:expr,)=>{};($sc:expr,$var:ident:$t:tt$($r:tt)*)=>{let $var=read_value!($sc,$t);input_inner!{$sc $($r)*}};}
    #[macro_export] macro_rules! read_value{($sc:expr,($($t:tt),*))=>{($(read_value!($sc,$t)),*)};($sc:expr,[$t:tt;$len:expr])=>{(0..$len).map(|_|read_value!($sc,$t)).collect::<Vec<_>>()};($sc:expr,Chars)=>{read_value!($sc,String).chars().collect::<Vec<char>>()};($sc:expr,Usize1)=>{read_value!($sc,usize)-1};($sc:expr,$t:ty)=>{$sc.next::<$t>()};}
    pub struct Scanner{s:Box<str>,input:std::iter::Peekable<std::str::SplitAsciiWhitespace<'static>>,}
    impl Scanner{
        pub fn new<R:std::io::Read>(mut reader:R)->Self{let mut sc=Scanner{s:{let mut s=String::new();reader.read_to_string(&mut s).unwrap();s.into_boxed_str()},input:"".split_ascii_whitespace().peekable(),};let s:&'static str=unsafe{std::mem::transmute(&*sc.s)};sc.input=s.split_ascii_whitespace().peekable();sc}
        #[inline]pub fn next<T:std::str::FromStr>(&mut self)->T where T::Err:std::fmt::Debug,{self.input.next().unwrap().parse::<T>().expect("Parse error")}
    }
}
#[proconio::fastout]
fn main() {
    input!(n: usize, xy: [(i64, i64); n]);
    let mut x_map = std::collections::BTreeMap::new();
    let mut y_map = std::collections::BTreeMap::new();

    for (i, &(x, y)) in xy.iter().enumerate() {
        x_map.entry(x).or_insert_with(|| vec![]).push(i);
        y_map.entry(y).or_insert_with(|| vec![]).push(i);
    }
    let mut index = std::collections::BTreeSet::new();
    for (k, v) in x_map.iter().take(2) {
        for j in v.iter().take(2) {
            index.insert(*j);
        }
        if v.len() >= 2 {
            break;
        }
    }
    for (k, v) in x_map.iter().rev().take(2) {
        for j in v.iter().take(2) {
            index.insert(*j);
        }
        if v.len() >= 2 {
            break;
        }
    }
    for (k, v) in y_map.iter().take(2) {
        for j in v.iter().take(2) {
            index.insert(*j);
        }
        if v.len() >= 2 {
            break;
        }
    }
    for (k, v) in y_map.iter().rev().take(2) {
        for j in v.iter().take(2) {
            index.insert(*j);
        }
        if v.len() >= 2 {
            break;
        }
    }

    let index = index.iter().cloned().collect::<Vec<_>>();

    let mut dist = vec![];
    for i in 0..index.len() {
        for j in i + 1..index.len() {
            dist.push(
                (xy[index[i]].0 - xy[index[j]].0)
                    .abs()
                    .max((xy[index[i]].1 - xy[index[j]].1).abs()),
            );
        }
    }
    dist.sort_unstable();
    println!("{}", dist[dist.len() - 2]);
}
