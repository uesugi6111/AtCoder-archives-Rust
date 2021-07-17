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
        e[a - 1].push(b as i64 - 1);
        e[b - 1].push(a as i64 - 1);
    }
    let lca = lca::LowestCommonAncestor::new(&e, 0);
    for (c, d) in cd {
        println!(
            "{}",
            if lca.get_distance(c - 1, d - 1, 0) % 2 == 0 {
                "Town"
            } else {
                "Road"
            }
        )
    }
}
mod lca {
    use std::collections::VecDeque;

    struct Node {
        pub parent: Option<usize>,
        pub number: usize,
        pub depth: i64,
    }

    impl Node {
        #[inline]
        pub fn new(parent: Option<usize>, number: usize, depth: i64) -> Self {
            Node {
                parent,
                number,
                depth,
            }
        }
    }

    pub struct LowestCommonAncestor {
        max_log_v: usize,
        depths: Vec<Option<i64>>,
        ancestors: Vec<Vec<Option<usize>>>,
    }

    impl LowestCommonAncestor {
        // 隣接リストで受け取る
        #[inline]
        pub fn new(edges: &[Vec<i64>], root: usize) -> Self {
            let max_v = edges.len();
            let max_log_v = ((max_v as f64).ln() / 2.0_f64.ln()) as usize + 1;
            let mut ancestors = vec![vec![None; max_v]; max_log_v + 1];
            let mut depths = vec![None; max_v];

            let mut q = VecDeque::new();
            q.push_back(Node::new(None, root, 0));
            while let Some(node) = q.pop_front() {
                if depths[node.number].is_some() {
                    continue;
                }
                ancestors[0][node.number] = node.parent;

                depths[node.number] = Some(node.depth);
                for i in 0..edges[node.number].len() {
                    if node.parent.is_none() || node.parent.unwrap() as i64 != edges[node.number][i]
                    {
                        q.push_back(Node::new(
                            Some(node.number),
                            edges[node.number][i] as usize,
                            node.depth + 1,
                        ));
                    }
                }
            }

            (0..max_log_v).for_each(|i| {
                (0..max_v).for_each(|j| {
                    if let Some(ancetor) = ancestors[i][j] {
                        ancestors[i + 1][j] = ancestors[i][ancetor];
                    }
                })
            });

            LowestCommonAncestor {
                max_log_v,
                depths,
                ancestors,
            }
        }
        #[inline]
        pub fn get_lca(&self, u: usize, v: usize) -> Option<usize> {
            let (mut u, mut v) = if self.depths[u].unwrap_or(0) > self.depths[v].unwrap_or(0) {
                (v, u)
            } else {
                (u, v)
            };

            for k in 0..self.max_log_v {
                if (((self.depths[v].unwrap_or(0) - self.depths[u].unwrap_or(0)) >> k) & 1) == 1 {
                    v = self.ancestors[k][v].unwrap();
                }
            }

            if u == v {
                return Some(u);
            }

            for k in (0..self.max_log_v).rev() {
                if self.ancestors[k][u].is_none()
                    || self.ancestors[k][v].is_none()
                    || self.ancestors[k][u] == self.ancestors[k][v]
                {
                    continue;
                }

                u = self.ancestors[k][u].unwrap();
                v = self.ancestors[k][v].unwrap();
            }
            self.ancestors[0][u]
        }
        #[inline]
        pub fn get_distance(&self, u: usize, v: usize, root: usize) -> i64 {
            let lca = self.get_lca(u, v).unwrap_or(root);
            self.depths[u].unwrap_or(0) + self.depths[v].unwrap_or(0)
                - self.depths[lca].unwrap_or(0) * 2
        }
    }

    #[cfg(test)]
    mod tests {

        use super::*;
        #[test]
        fn test_lca() {
            let n = 5;
            let mut e = vec![vec![]; n];
            for (i, &v) in [0, 0, 2, 2].iter().enumerate() {
                e[v].push(i as i64 + 1);
            }

            let lca = LowestCommonAncestor::new(&e, 0);
            for &(u, v, ans) in [(0, 1, 0), (0, 4, 0), (1, 2, 0), (2, 3, 2), (3, 4, 2)].iter() {
                assert_eq!(lca.get_lca(u, v).unwrap_or(0), ans);
            }
        }
    }
}
