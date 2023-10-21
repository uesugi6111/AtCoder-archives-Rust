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
    input!(h: usize, w: usize, s: [Chars; h]);

    let mut dsu = dsu::DisjointSetUnion::new(h * w);
    let mut v = vec![];

    for i in 0..h {
        for j in 0..w {
            if s[i][j] != '#' {
                continue;
            }
            v.push(i * w + j);

            if i != 0 {
                if j != 0 && s[i - 1][j - 1] == '#' {
                    dsu.unite(i * w + j, (i - 1) * w + (j - 1));
                }
                if s[i - 1][j] == '#' {
                    dsu.unite(i * w + j, (i - 1) * w + (j));
                }
                if j != w - 1 && s[i - 1][j + 1] == '#' {
                    dsu.unite(i * w + j, (i - 1) * w + (j + 1));
                }
            }
            if j != 0 && s[i][j - 1] == '#' {
                dsu.unite(i * w + j, (i) * w + (j - 1));
            }

            if j != w - 1 && s[i][j + 1] == '#' {
                dsu.unite(i * w + j, (i) * w + (j + 1));
            }
            if i != h - 1 {
                if j != 0 && s[i + 1][j - 1] == '#' {
                    dsu.unite(i * w + j, (i + 1) * w + (j - 1));
                }
                if s[i + 1][j] == '#' {
                    dsu.unite(i * w + j, (i + 1) * w + (j));
                }
                if j != w - 1 && s[i + 1][j + 1] == '#' {
                    dsu.unite(i * w + j, (i + 1) * w + (j + 1));
                }
            }
        }
    }

    let mut set = std::collections::HashSet::new();

    for i in v {
        let buff = dsu.find_root(i);
        set.insert(buff);
    }

    println!("{}", set.len());
}

mod dsu {
    //! Union find
    use std::collections::{HashMap, HashSet};
    #[derive(Copy, Clone, Debug)]
    enum Node {
        Root(usize),
        Child(usize),
    }
    ///UnionFind
    #[derive(Clone, Debug)]
    pub struct DisjointSetUnion {
        nodes: Vec<Node>,
    }

    impl DisjointSetUnion {
        pub fn new(n: usize) -> DisjointSetUnion {
            DisjointSetUnion {
                nodes: vec![Node::Root(1); n],
            }
        }

        pub fn find_root(&mut self, target: usize) -> usize {
            match unsafe { *self.nodes.get_unchecked(target) } {
                Node::Root(_) => target,
                Node::Child(parent) => {
                    let parent_index = self.find_root(parent);
                    self.nodes[target] = Node::Child(parent_index);
                    parent_index
                }
            }
        }
        pub fn unite(&mut self, x: usize, y: usize) -> bool {
            let rx = self.find_root(x);
            let ry = self.find_root(y);
            if rx == ry {
                return false;
            }
            let size_x = self.size(x);
            let size_y = self.size(y);

            let (i, j) = if size_x > size_y { (rx, ry) } else { (ry, rx) };
            self.nodes[i] = Node::Root(size_x + size_y);
            self.nodes[j] = Node::Child(i);

            true
        }
        pub fn is_same(&mut self, x: usize, y: usize) -> bool {
            self.find_root(x) == self.find_root(y)
        }
        pub fn size(&mut self, x: usize) -> usize {
            let root = self.find_root(x);
            match self.nodes[root] {
                Node::Root(size) => size,
                Node::Child(_) => 0,
            }
        }
        pub fn get_same_group(&mut self, x: usize) -> HashSet<usize> {
            let root = self.find_root(x);
            let mut g = HashSet::new();
            for i in 0..self.nodes.len() {
                if root == self.find_root(i) {
                    g.insert(i);
                }
            }
            g
        }
        pub fn get_all_groups(&mut self) -> HashMap<usize, HashSet<usize>> {
            let mut map = HashMap::new();
            for i in 0..self.nodes.len() {
                map.entry(self.find_root(i))
                    .or_insert_with(HashSet::new)
                    .insert(i);
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
