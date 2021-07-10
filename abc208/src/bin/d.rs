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
    input!(n: usize, m: usize, abc: [(i64, i64, i64); m]);

    let e = {
        let mut e = vec![vec![]; n];
        for &(a, b, c) in abc.iter() {
            e[a as usize - 1].push((b as usize - 1, c));
        }
        e
    };
    let mut dsu = dsu::Dsu::new(n + 2);
    for &(a, b, c) in abc.iter() {
        dsu.unite(a as usize, b as usize);
    }

    let mut count = 0;
    for i in 0..n {
        for j in 0..n {
            if i == j {
                continue;
            }
            if !dsu.is_same(i + 1, j + 1) {
                continue;
            }

            let mut k = n as i64 - 1;
            while k >= 0 {
                if let Some((cost, min_node)) = dijkstra::dijkstra(&e, i, j, k as usize) {
                    count += cost * (k - min_node.unwrap_or(0) as i64 + 1);
                    if min_node == None {
                        break;
                    }
                    k = min_node.unwrap() as i64 - 1;
                } else {
                    break;
                }
            }
        }
    }

    println!("{}", count);
}
mod dijkstra {
    //! ダイクストラ

    use std::{cmp::Ordering, collections::BinaryHeap};

    #[derive(Debug, Clone, Eq)]
    pub struct Node {
        position: usize,
        cost: i64,
        min_node: Option<usize>,
    }
    impl Node {
        #[inline]
        pub fn new(position: usize, cost: i64, min_node: Option<usize>) -> Self {
            Node {
                position,
                cost,
                min_node,
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
        k: usize,
    ) -> Option<(i64, Option<usize>)> {
        assert_ne!(start, end);
        let mut costs = vec![None; edge.len()];
        let mut nodes = BinaryHeap::new();
        nodes.push(Node::new(start, 0, None));

        while let Some(Node {
            position,
            cost,
            min_node,
        }) = nodes.pop()
        {
            if costs[position].is_some() {
                continue;
            }
            if position == end {
                return Some((cost, min_node));
            }
            costs[position] = Some(cost);

            edge[position]
                .iter()
                .filter(|(to, c)| *to == end || *to <= k)
                .filter(|(to, c)| costs[*to].filter(|&d| d <= cost + c).is_none())
                .for_each(|&(to, c)| {
                    nodes.push(Node::new(
                        to,
                        cost + c,
                        if min_node.is_none() {
                            if to != end {
                                Some(to)
                            } else {
                                min_node
                            }
                        } else if to != end {
                            Some(min_node.unwrap().max(to))
                        } else {
                            min_node
                        },
                    ));
                });
        }
        None
    }
}

mod dsu {
    //! Union find
    use std::collections::{HashMap, HashSet};
    #[derive(Debug, Clone)]
    enum Node {
        Root(usize),
        Child(usize),
    }
    ///UnionFind
    #[derive(Clone, Debug)]
    pub struct Dsu {
        uf: Vec<Node>,
    }

    impl Dsu {
        pub fn new(n: usize) -> Dsu {
            Dsu {
                uf: vec![Node::Root(1); n],
            }
        }

        pub fn root(&mut self, target: usize) -> usize {
            match self.uf[target] {
                Node::Root(_) => target,
                Node::Child(par) => {
                    let root = self.root(par);
                    self.uf[target] = Node::Child(self.root(par));
                    root
                }
            }
        }
        pub fn unite(&mut self, x: usize, y: usize) -> bool {
            let rx = self.root(x);
            let ry = self.root(y);
            if rx == ry {
                return false;
            }
            let size_x = self.size(x);
            let size_y = self.size(y);

            let (i, j) = if size_x > size_y { (rx, ry) } else { (ry, rx) };
            self.uf[i] = Node::Root(size_x + size_y);
            self.uf[j] = Node::Child(i);

            true
        }
        pub fn is_same(&mut self, x: usize, y: usize) -> bool {
            self.root(x) == self.root(y)
        }
        pub fn size(&mut self, x: usize) -> usize {
            let root = self.root(x);
            match self.uf[root] {
                Node::Root(size) => size,
                Node::Child(_) => 0,
            }
        }
        pub fn get_same_group(&mut self, x: usize) -> HashSet<usize> {
            let root = self.root(x);
            let mut g = HashSet::new();
            for i in 0..self.uf.len() {
                if root == self.root(i) {
                    g.insert(i);
                }
            }
            g
        }
        pub fn get_all_groups(&mut self) -> HashMap<usize, HashSet<usize>> {
            let mut map: HashMap<usize, HashSet<usize>> = HashMap::new();
            for i in 0..self.uf.len() {
                let root = self.root(i);

                map.entry(root).or_insert_with(HashSet::new).insert(i);
            }
            map
        }
    }
}
