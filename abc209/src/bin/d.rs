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
    input!(
        n: usize,
        q: usize,
        ab: [(usize, usize); n - 1],
        cd: [(usize, usize); q]
    );

    let mut v = vec![0_i64; n];
    let mut ab = ab;
    ab.sort();

    let mut e = vec![vec![]; n];
    for &(a, b) in ab.iter() {
        e[a - 1].push(b - 1);
        e[b - 1].push(a - 1);
    }
    let v = dijk::dijkstra(&e, 0);

    for (c, d) in cd {
        println!(
            "{}",
            if (v[c - 1] + v[d - 1]) % 2 == 0 {
                "Town"
            } else {
                "Road"
            }
        )
    }
}

mod dijk {
    #[derive(Debug, Clone, PartialEq, Eq, Ord)]
    struct Node {
        pos: usize,
        cost: i64,
    }

    impl PartialOrd for Node {
        fn partial_cmp(&self, other: &Node) -> Option<std::cmp::Ordering> {
            Some(other.cost.cmp(&(self.cost)))
        }
    }

    pub fn dijkstra(edge: &[Vec<usize>], start: usize) -> Vec<i64> {
        let mut dist = vec![std::i64::MAX; edge.len()];
        let mut pq = std::collections::BinaryHeap::<Node>::new();

        pq.push(Node {
            pos: start,
            cost: 0,
        });
        dist[start] = 0;

        while let Some(Node { pos, cost }) = pq.pop() {
            if cost > dist[pos] {
                continue;
            }
            for t in &edge[pos] {
                let total_cost = cost + 1;
                if dist[*t] <= total_cost {
                    continue;
                }
                dist[*t] = total_cost;
                pq.push(Node {
                    pos: *t,
                    cost: total_cost,
                });
            }
        }
        dist
    }

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
        assert_eq!(dijkstra(&graph, 0, 3), Some(3));
        assert_eq!(dijkstra(&graph, 3, 0), Some(7));
        assert_eq!(dijkstra(&graph, 0, 4), Some(5));
        assert_eq!(dijkstra(&graph, 4, 0), None);
    }
}
