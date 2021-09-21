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
use std::io::{stdout, BufWriter, Write};
fn main() {
    input!(
        n: usize,
        m: usize,
        s: usize,
        t: usize,
        abc: [(usize, usize, i64); m]
    );
    let mut e = vec![vec![]; n];
    abc.iter().for_each(|&(a, b, c)| {
        e[a].push((b, c));
    });
    let out = stdout();
    let mut out_lock = BufWriter::new(out.lock());

    if let Some((c, p)) = dijk::dijkstra(&e, s, t, n) {
        writeln!(&mut out_lock, "{} {}", c, p.len() - 1).ok();
        for i in 0..p.len() - 1 {
            writeln!(&mut out_lock, "{} {}", p[i], p[i + 1]).ok();
        }
    } else {
        writeln!(&mut out_lock, "-1").ok();
    }
}

mod dijk {
    //! ダイクストラ
    use std::{cmp::Ordering, collections::BinaryHeap};

    #[derive(Debug, Clone, Eq)]
    pub struct Node {
        position: usize,
        cost: i64,
        from: Option<usize>,
    }
    impl Node {
        #[inline]
        pub fn new(position: usize, cost: i64, from: Option<usize>) -> Self {
            Node {
                position,
                cost,
                from,
            }
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
        edge: &[Vec<(usize, i64)>],
        start: usize,
        end: usize,
        vertex: usize,
    ) -> Option<(i64, Vec<usize>)> {
        assert_ne!(start, end);
        let mut costs = vec![None; edge.len()];
        let mut nodes = BinaryHeap::new();
        let mut previous = vec![None; vertex];
        nodes.push(Node::new(start, 0, None));

        while let Some(Node {
            position,
            cost,
            from,
        }) = nodes.pop()
        {
            if costs[position].is_some() {
                continue;
            }

            previous[position] = from;
            costs[position] = Some(cost);

            if position == end {
                return Some((cost, restore_path(end, &previous)));
            }

            for &(to, c) in &edge[position] {
                let total_cost = cost + c;
                if costs[to].filter(|&d| d <= total_cost).is_some() {
                    continue;
                }

                nodes.push(Node::new(to, total_cost, Some(position)));
            }
        }
        None
    }

    fn restore_path(end: usize, previous: &[Option<usize>]) -> Vec<usize> {
        let mut buff = end;
        let mut v = vec![buff];

        while let Some(i) = previous[buff] {
            buff = i;
            v.push(buff);
        }
        v.reverse();
        v
    }
}
