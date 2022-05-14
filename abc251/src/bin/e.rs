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
    input!(n: usize, s: [i64; n]);

    let mut a = s
        .iter()
        .enumerate()
        .map(|(i, v)| (v, i, (i + 1) % n))
        .collect::<Vec<_>>();

    let mut e = vec![vec![]; n];
    for i in 0..n {
        e[i].push(((i + 1) % n, s[i]));
        e[(i + 1) % n].push((i, s[i]));
    }
    let ans = dijk::dijkstra(&e, 0, 0).unwrap();

    let mut sum = 0;

    println!("{}", ans);
}

mod dijk {

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

    pub fn dijkstra(edge: &[Vec<(usize, i64)>], start: usize, end: usize) -> Option<i64> {
        //assert_ne!(start, end);
        let mut costs = vec![None; edge.len()];
        let mut nodes = BinaryHeap::new();
        nodes.push(Node::new(start, 0));

        let mut is = false;

        while let Some(Node { position, cost }) = nodes.pop() {
            if costs[position].is_some() {
                continue;
            }
            costs[position] = Some(cost);
            if position == end && is {
                return Some(cost);
            } else if position == end && !is {
                is = true;
                costs[position] = None;
            }

            edge[position]
                .iter()
                .filter(|(to, c)| costs[*to].filter(|&d| d <= cost + c).is_none())
                .for_each(|&(to, c)| {
                    nodes.push(Node::new(to, cost + c));
                });
        }
        None
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        #[test]
        fn test_dijkstra() {
            let graph = vec![
                vec![(2, 10), (1, 1)],
                vec![(3, 2)],
                vec![(1, 1), (3, 3), (4, 1)],
                vec![(0, 7), (4, 2)],
                vec![],
            ];

            assert_eq!(dijkstra(&graph, 0, 1), Some(1));
            assert_eq!(dijkstra(&graph, 0, 2), Some(10));
            assert_eq!(dijkstra(&graph, 0, 3), Some(3));
            assert_eq!(dijkstra(&graph, 0, 4), Some(5));
            assert_eq!(dijkstra(&graph, 3, 0), Some(7));
            assert_eq!(dijkstra(&graph, 4, 0), None);
        }
        #[test]
        #[should_panic]
        fn test_panic() {
            dijkstra(&[], 0, 0);
        }
    }
}
