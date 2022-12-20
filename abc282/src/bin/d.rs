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
    input!(n: usize, m: usize, uv: [(i64, i64); m]);

    let mut g = vec![vec![]; n];
    let mut dsu = dsu::DisjointSetUnion::new(n);

    for (u, v) in uv {
        g[u as usize - 1].push(v as usize - 1);
        g[v as usize - 1].push(u as usize - 1);
        dsu.unite(u as usize - 1, v as usize - 1);
    }

    let mut rec = Rec {
        g,
        color: vec![0; n],
    };

    let aa = dsu.get_all_groups();

    let mut is_ok = true;
    for (i, (&k, v)) in aa.iter().enumerate() {
        if !rec.solve(k, 1 + i as i64) {
            is_ok = false;
            break;
        }
    }
    if !is_ok {
        println!("0");
        return;
    }
    let mut map = std::collections::HashMap::new();
    for c in rec.color.iter() {
        *map.entry(*c).or_insert(0) += 1;
    }
    let mut co = vec![];
    for i in 1..n as i64 {
        if map.get(&(i)).is_none() {
            break;
        }

        let (aa, bb) = if map.get(&(i)).unwrap() < map.get(&(-i)).unwrap_or(&0) {
            (map.get(&(i)).unwrap(), map.get(&(-i)).unwrap_or(&0))
        } else {
            (map.get(&(-i)).unwrap_or(&0), map.get(&(i)).unwrap())
        };
        co.push((*bb, *aa));
    }
    let mut sum = 0;
    for (a, b) in co {
        sum += a * (a - 1);
        sum += b * (b - 1);
    }

    println!("{}", sum / 2 - m);
}

struct Rec {
    g: Vec<Vec<usize>>,
    pub color: Vec<i64>,
}
impl Rec {
    fn solve(&mut self, u: usize, color: i64) -> bool {
        self.color[u] = color;

        let mut ok = true;
        for i in 0..self.g[u].len() {
            let v = self.g[u][i];
            if self.color[v] == 0 {
                if !self.solve(v, -1 * color) {
                    ok = false
                }
            } else {
                if self.color[v] == color {
                    ok = false
                }
            }
        }
        ok
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
