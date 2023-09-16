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
    input!(n: usize, m: usize, tws: [(i64, i64, i64); m]);

    let mut bh = BinaryHeap::new();
    let mut hito_bh = BinaryHeap::new();

    for (t, w, s) in tws {
        bh.push(Node::new(None, t, Some((w, s))));
    }

    for i in 0..n {
        hito_bh.push(std::cmp::Reverse(i));
    }
    let mut ans = vec![0; n];

    while let Some(x) = bh.pop() {
        if let Some(xx) = x.hito {
            hito_bh.push(std::cmp::Reverse(xx));
        }

        if let Some(ws) = x.ws {
            if let Some(xxx) = hito_bh.pop() {
                ans[xxx.0] += ws.0;
                bh.push(Node::new(Some(xxx.0), x.cost + ws.1, None));
            }
        }
    }
    for i in ans {
        println!("{}", i);
    }
}

use std::{cmp::Ordering, collections::BinaryHeap};

#[derive(Debug, Clone, Eq)]
pub struct Node {
    hito: Option<usize>,
    cost: i64,
    ws: Option<(i64, i64)>,
    cc: i64,
}
impl Node {
    #[inline]
    pub fn new(hito: Option<usize>, cost: i64, ws: Option<(i64, i64)>) -> Self {
        Node {
            hito,
            cost,
            ws,
            cc: cost * 10 + if hito.is_none() { 5 } else { 0 },
        }
    }
}
impl PartialEq for Node {
    fn eq(&self, other: &Node) -> bool {
        self.cc.eq(&other.cc)
    }
}
impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Node) -> Option<Ordering> {
        Some(other.cc.cmp(&(self.cc)))
    }
}
impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        self.cc.cmp(&(other.cc))
    }
}
