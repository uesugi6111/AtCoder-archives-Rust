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
    input!(n: usize, m: usize, st: [(usize, usize); m]);
    let mut e = vec![vec![]; n];
    for (i, &(s, t)) in st.iter().enumerate() {
        e[s - 1].push((t - 1, 1, i + 1));
    }
    for i in 0..m {
        println!("{}", djik::dijkstra(&e, 0, n - 1, i + 1).unwrap_or(-1));
    }
}

mod djik {
    //! ダイクストラ

    use std::{cmp::Ordering, collections::BinaryHeap};

    #[derive(Debug, Clone, Eq)]
    pub struct Node {
        position: usize,
        cost: i64,
    }
    impl Node {
        #[inline]
        pub fn new(position: usize, cost: i64) -> Self {
            Node { position, cost }
        }
    }
    impl PartialEq for Node {
        fn eq(&self, other: &Node) -> bool {
            self.cost.eq(&other.cost)
        }
    }
    impl PartialOrd for Node {
        fn partial_cmp(&self, other: &Node) -> Option<Ordering> {
            Some(other.cost.cmp(&(self.cost)))
        }
    }
    impl Ord for Node {
        fn cmp(&self, other: &Self) -> Ordering {
            self.cost.cmp(&(other.cost))
        }
    }

    pub fn dijkstra(
        edge: &[Vec<(usize, i64, usize)>],
        start: usize,
        end: usize,
        unused: usize,
    ) -> Option<i64> {
        assert_ne!(start, end);
        let mut costs = vec![None; edge.len()];
        let mut nodes = std::collections::VecDeque::new();
        nodes.push_back(Node::new(start, 0));

        while let Some(Node { position, cost }) = nodes.pop_front() {
            if costs[position].is_some() {
                continue;
            }
            if position == end {
                return Some(cost);
            }
            costs[position] = Some(cost);

            edge[position]
                .iter()
                .filter(|e| e.2 != unused)
                .filter(|(to, c, _)| costs[*to].filter(|&d| d <= cost + c).is_none())
                .for_each(|&(to, c, _)| {
                    nodes.push_back(Node::new(to, cost + c));
                });
        }
        None
    }
}
