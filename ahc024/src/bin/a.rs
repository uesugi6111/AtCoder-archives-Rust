use std::time::Instant;
#[rustfmt::skip]
mod io_pro {
    #[macro_export] macro_rules! input{(sc=$sc:expr,$($r:tt)*)=>{input_inner!{$sc,$($r)*}};($($r:tt)*)=>{let mut sc=io_pro::Scanner::new(std::io::BufReader::new(std::io::stdin().lock()));input_inner!{sc,$($r)*}};}
    #[macro_export] macro_rules! input_inner{($sc:expr)=>{};($sc:expr,)=>{};($sc:expr,$var:ident:$t:tt$($r:tt)*)=>{let $var=read_value!($sc,$t);input_inner!{$sc $($r)*}};($sc:expr,mut $var:ident:$t:tt$($r:tt)*)=>{let mut $var=read_value!($sc,$t);input_inner!{$sc $($r)*}};}
    #[macro_export] macro_rules! read_value{($sc:expr,($($t:tt),*))=>{($(read_value!($sc,$t)),*)};($sc:expr,[$t:tt;$len:expr])=>{(0..$len).map(|_|read_value!($sc,$t)).collect::<std::collections::VecDeque<_>>()};($sc:expr,Chars)=>{read_value!($sc,String).chars().collect::<Vec<char>>()};($sc:expr,Usize1)=>{read_value!($sc,usize)-1};($sc:expr,$t:ty)=>{$sc.next::<$t>()};}
    pub struct Scanner{s:Box<str>,input:std::str::SplitAsciiWhitespace<'static>,}
    impl Scanner{
        pub fn new<R:std::io::Read>(mut reader:R)->Self{let mut sc=Scanner{s:{let mut s=String::new();reader.read_to_string(&mut s).unwrap();s.into_boxed_str()},input:"".split_ascii_whitespace(),};let s:&'static str=unsafe{std::mem::transmute(&*sc.s)};sc.input=s.split_ascii_whitespace();sc}
        #[inline]pub fn next<T:std::str::FromStr>(&mut self)->T where T::Err:std::fmt::Debug,{self.input.next().unwrap().parse::<T>().expect("Parse error")}
    }
}
#[proconio::fastout]
fn main() {
    let since = Instant::now();
    let mut time = Instant::now().duration_since(since).as_millis();
    input!(n: usize, m: usize, c: [[i8; n]; n]);

    let base = {
        let mut base = vec![vec![false; m + 1]; m + 1];
        for i in 0..m {
            base[i][i] = true;
        }

        let mut dsu = dsu::DisjointSetUnion::new(n * n + 1);

        for i in 0..n {
            for j in 0..n - 1 {
                if c[i][j] == c[i][j + 1] {
                    dsu.unite(i * n + j, i * n + j + 1);
                }

                base[c[i][j].min(c[i][j + 1]) as usize][c[i][j].max(c[i][j + 1]) as usize] = true;

                if c[j][i] == c[j + 1][i] {
                    dsu.unite(j * n + i, (j + 1) * n + i);
                }

                base[c[j][i].min(c[j + 1][i]) as usize][c[j][i].max(c[j + 1][i]) as usize] = true;
            }
        }

        for i in 0..n {
            base[0][c[0][i] as usize] = true;
            base[0][c[n - 1][i] as usize] = true;
            base[0][c[i][0] as usize] = true;
            base[0][c[i][n - 1] as usize] = true;
        }
        (base, dsu.get_all_groups().len())
    };

    let mut cc = c.clone();
    let mut count = 0;
    let mut xs = xorshift::XorShift::<u32>::new();
    'co: while time < 1980 {
        let (x, y) = (
            xs.next().unwrap() as usize % n,
            xs.next().unwrap() as usize % n,
        );
        count += 1;
        if (count & ((1 << 6) - 1)) == 0 {
            time = Instant::now().duration_since(since).as_millis();
        }
        if cc[y][x] == 0 {
            continue;
        }
        let mut ccc = cc.clone();

        if xs.next().unwrap() % 2 == 0 {
            let r = ccc[y].remove(x).unwrap();
            if r == 0 {
                continue;
            }
            if xs.next().unwrap() % 2 == 0 {
                ccc[y].push_front(0);
            } else {
                ccc[y].push_back(0);
            }
        } else {
            if ccc[y][x] == 0 {
                continue;
            }
            if xs.next().unwrap() % 2 == 0 {
                for i in 0..y {
                    ccc[y - i][x] = ccc[y - i - 1][x];
                }

                ccc[0][x] = 0;
            } else {
                for i in y..n - 1 {
                    ccc[i][x] = ccc[i + 1][x];
                }
                ccc[n - 1][x] = 0;
            }
        }

        for i in 1..n - 1 {
            for j in 1..n - 1 {
                if ccc[i][j] == 0 {
                    if ccc[i - 1][j] != 0
                        && ccc[i + 1][j] != 0
                        && ccc[i][j - 1] != 0
                        && ccc[i][j + 1] != 0
                    {
                        continue 'co;
                    }
                }
            }
        }

        let tmp = {
            let mut base = vec![vec![false; m + 1]; m + 1];
            for i in 0..m {
                base[i][i] = true;
            }

            let mut dsu = dsu::DisjointSetUnion::new(n * n + 1);
            for i in 0..n {
                for j in 0..n - 1 {
                    if ccc[i][j] == ccc[i][j + 1] {
                        dsu.unite(i * n + j, i * n + j + 1);
                    }

                    base[ccc[i][j].min(ccc[i][j + 1]) as usize]
                        [ccc[i][j].max(ccc[i][j + 1]) as usize] = true;

                    if ccc[j][i] == ccc[j + 1][i] {
                        dsu.unite(j * n + i, (j + 1) * n + i);
                    }
                    base[ccc[j][i].min(ccc[j + 1][i]) as usize]
                        [ccc[j][i].max(ccc[j + 1][i]) as usize] = true;
                }
            }
            for i in 0..n {
                if ccc[0][i] == 0 {
                    dsu.unite(n * n, i);
                }
                if ccc[n - 1][i] == 0 {
                    dsu.unite(n * n, n * (n - 1) + i);
                }
                if ccc[i][0] == 0 {
                    dsu.unite(n * n, i * n);
                }
                if ccc[i][n - 1] == 0 {
                    dsu.unite(n * n, i * n + n - 1);
                }
            }
            for i in 0..n {
                base[0][ccc[0][i] as usize] = true;
                base[0][ccc[n - 1][i] as usize] = true;
                base[0][ccc[i][0] as usize] = true;
                base[0][ccc[i][n - 1] as usize] = true;
            }
            (base, dsu.get_all_groups().len())
        };
        if base == tmp {
            cc = ccc;
        }
    }

    for i in 0..n {
        for j in 0..n {
            print!("{} ", cc[i][j]);
        }
        println!();
    }
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

mod xorshift {
    //! Xorshift random number generator
    use std::{
        fmt::{Debug, Display},
        time::SystemTime,
    };

    #[derive(Clone, Default, Copy, Debug)]
    pub struct XorShift<T>
    where
        T: std::fmt::Debug + Sized + Copy + Display + Shift,
    {
        seed: T,
    }

    impl<T> XorShift<T>
    where
        T: std::fmt::Debug + Sized + Copy + Display + Shift,
    {
        pub fn new() -> Self {
            XorShift::from_seed(T::seed())
        }
        pub fn from_seed(seed: T) -> XorShift<T> {
            XorShift { seed }
        }
    }

    impl<T> Iterator for XorShift<T>
    where
        T: std::fmt::Debug + Sized + Copy + Display + Shift,
    {
        type Item = T;

        fn next(&mut self) -> Option<Self::Item> {
            T::shift(&mut self.seed);
            Some(self.seed)
        }
    }

    pub trait Shift {
        fn seed() -> Self;
        fn shift(n: &mut Self);
    }

    impl Shift for u64 {
        fn seed() -> Self {
            SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs()
        }

        fn shift(state: &mut u64) {
            *state ^= *state << 13;
            *state ^= *state >> 7;
            *state ^= *state << 17;
        }
    }
    impl Shift for u32 {
        fn seed() -> Self {
            SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs() as u32
        }

        fn shift(state: &mut u32) {
            *state ^= *state << 13;
            *state ^= *state >> 17;
            *state ^= *state << 5;
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use std::collections::HashSet;
        #[test]
        fn test_xorshift() {
            let mut set = HashSet::new();
            let xorshift = XorShift::<u64>::new();

            for v in xorshift.take(100_000) {
                assert!(!set.contains(&v));
                set.insert(v);
            }
        }
    }
}
