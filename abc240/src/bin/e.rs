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
use std::collections::VecDeque;
#[proconio::fastout]
fn main() {
    input!(n: usize, uv: [(usize, usize); n - 1]);

    let mut e = vec![vec![]; n];

    for (u, v) in uv {
        e[u - 1].push(v - 1);
        e[v - 1].push(u - 1);
    }
    let mut count = 1;
    let mut ans: Vec<Option<(i64, i64)>> = vec![None; n];
    let mut stack = VecDeque::new();
    stack.push_back(Vertex::Out(0));
    stack.push_back(Vertex::In(0));
    let mut tour = vec![];
    let mut first_look = vec![None; e.len()];
    let mut depth = 0;
    let mut depths = vec![0; e.len()];
    while let Some(vertex) = stack.pop_back() {
        match vertex {
            Vertex::In(v) => {
                for &to in e[v].iter() {
                    if first_look[to].is_some() {
                        continue;
                    }
                    stack.push_back(Vertex::Out(to));
                    stack.push_back(Vertex::In(to));
                }
                first_look[v] = Some(tour.len());
                depths[v] = depth;
                depth += 1;
            }
            Vertex::Out(v) => {
                let (mut l, mut r) = (None, None);
                for &to in e[v].iter() {
                    if depths[to] != depth {
                        continue;
                    }
                    if l.is_none() {
                        l = Some(ans[to].unwrap().0);
                    } else {
                        l = Some(ans[to].unwrap().0.min(l.unwrap()));
                    }
                    if r.is_none() {
                        r = Some(ans[to].unwrap().1);
                    } else {
                        r = Some(ans[to].unwrap().1.max(r.unwrap()));
                    }
                }
                if l.is_none() {
                    l = Some(count);
                    r = Some(count);
                    count += 1;
                }

                ans[v] = Some((l.unwrap(), r.unwrap()));

                depth -= 1;
            }
        }

        tour.push(vertex.get_value());
    }

    for i in 0..n {
        println!("{} {}", ans[i].unwrap().0, ans[i].unwrap().1);
    }
}
#[derive(Debug)]
pub enum Vertex {
    In(usize),
    Out(usize),
}
impl Vertex {
    pub fn get_value(&self) -> usize {
        match self {
            Vertex::In(value) => *value,
            Vertex::Out(value) => *value,
        }
    }
}
