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
    input!(
        n: usize,
        m: usize,
        q: usize,
        abc: [(i64, i64, i64); m],
        uvw: [(i64, i64, i64); q]
    );

    let mut e = abc
        .iter()
        .map(|x| Edge(x.0 - 1, x.1 - 1, x.2, false, 0))
        .chain(
            uvw.iter()
                .enumerate()
                .map(|(i, x)| Edge(x.0 - 1, x.1 - 1, x.2, true, i)),
        )
        .collect::<Vec<Edge>>();

    // dbg!(&e);

    let ans = mst::kruskal(n, &e, q);

    for i in 0..q {
        println!("{}", if ans[i] { "Yes" } else { "No" });
    }
}
#[derive(Debug)]
pub struct Edge(i64, i64, i64, bool, usize);
mod mst {
    use super::dsu::DisjointSetUnion;

    pub fn kruskal(n: usize, edges: &[super::Edge], q: usize) -> Vec<bool> {
        let mut edges = edges.iter().collect::<Vec<_>>();
        edges.sort_by_key(|e| e.2);

        let mut dsu = DisjointSetUnion::new(n);

        let mut min_cost = 0;

        let mut ans = vec![false; q + 1];

        for e in edges.iter() {
            if dsu.is_same(e.0 as usize, e.1 as usize) {
                continue;
            }

            if e.3 {
                ans[e.4] = true;
                continue;
            }
            dsu.unite(e.0 as usize, e.1 as usize);

            min_cost += e.2;
        }

        ans
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        #[test]
        fn test_kruskal() {
            let ans = kruskal(
                5,
                &[
                    Edge(0, 1, 10),
                    Edge(0, 3, 5),
                    Edge(1, 2, 1),
                    Edge(1, 3, 1000),
                    Edge(1, 4, 500),
                    Edge(2, 3, 100),
                    Edge(2, 4, 10000),
                    Edge(3, 4, 5000),
                ],
            );

            assert_eq!(ans, 516);
        }
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
    pub struct DisjointSetUnion {
        uf: Vec<Node>,
    }

    impl DisjointSetUnion {
        pub fn new(n: usize) -> DisjointSetUnion {
            DisjointSetUnion {
                uf: vec![Node::Root(1); n],
            }
        }

        pub fn root(&mut self, target: usize) -> usize {
            match self.uf[target] {
                Node::Root(_) => target,
                Node::Child(par) => {
                    let root = self.root(par);
                    self.uf[target] = Node::Child(root);
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

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_dsu() {
            let mut d = DisjointSetUnion::new(4);
            d.unite(0, 1);
            assert!(d.is_same(0, 1));
            d.unite(1, 2);
            assert!(d.is_same(0, 2));
            assert_eq!(d.size(0), 3);
            assert!(!d.is_same(0, 3));

            // assert_eq!(d.get_all_groups(), vec![vec![0, 1, 2], vec![3]]);
        }
    }
}
