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
use std::io::Write;
fn main() {
    input!(
        n: usize,
        q: usize,
        a: [usize; n - 1],
        lr: [(usize, usize); q]
    );

    let mut e = vec![vec![]; n];
    for (i, &v) in a.iter().enumerate() {
        e[v].push(i + 1);
    }
    let out = std::io::stdout();
    let mut out = std::io::BufWriter::new(out.lock());
    let lca = lca::LowestCommonAncestor::new(&e, 0);
    for &(u, v) in lr.iter() {
        writeln!(&mut out, "{}", lca.get_lca(u, v)).ok();
    }
}

mod lca {

    struct MinDepth {}
    impl sparse_table::Band for MinDepth {
        type T = (i32, i32);

        fn operate(a: &Self::T, b: &Self::T) -> Self::T {
            if a.1 < b.1 {
                *a
            } else {
                *b
            }
        }
    }

    pub struct LowestCommonAncestor {
        st: sparse_table::SparseTable<MinDepth>,
        first_look: Vec<usize>,
    }

    impl LowestCommonAncestor {
        pub fn new(e: &[Vec<usize>], root: usize) -> Self {
            let (tour, first_look, depths) = euler_tour::euler_tour(e, root);
            let v: Vec<(i32, i32)> = tour.iter().map(|&x| (x as i32, depths[x] as i32)).collect();
            let st = sparse_table::SparseTable::<MinDepth>::new(&v);

            LowestCommonAncestor { st, first_look }
        }

        pub fn get_lca(&self, u: usize, v: usize) -> usize {
            let range = if self.first_look[u] < self.first_look[v] {
                self.first_look[u]..self.first_look[v]
            } else {
                self.first_look[v]..self.first_look[u]
            };
            self.st.fold(range).0 as usize
        }
    }

    mod euler_tour {
        #[derive(Debug)]
        pub enum Vertex {
            In(usize),
            Out(usize),
        }
        impl Vertex {
            pub fn get_value(&self) -> usize {
                match self {
                    Vertex::In(value) => *value,
                    Vertex::Out(value) => *value,
                }
            }
        }
        use std::collections::VecDeque;

        pub fn euler_tour(e: &[Vec<usize>], root: usize) -> (Vec<usize>, Vec<usize>, Vec<usize>) {
            let mut stack = VecDeque::new();
            stack.push_back(Vertex::In(root));
            let mut tour = vec![];
            let mut first_look = vec![None; e.len()];
            let mut depth = 0;
            let mut depths = vec![0; e.len()];
            while let Some(vertex) = stack.pop_back() {
                if let Vertex::In(v) = vertex {
                    for &to in e[v].iter() {
                        if first_look[to].is_some() {
                            continue;
                        }
                        stack.push_back(Vertex::Out(v));
                        stack.push_back(Vertex::In(to));
                    }
                    first_look[v] = Some(tour.len());
                    depths[v] = depth;
                    depth += 1;
                } else {
                    depth -= 1;
                }
                tour.push(vertex.get_value());
            }

            (
                tour,
                first_look.iter().map(|x| x.unwrap()).collect(),
                depths,
            )
        }
    }

    mod sparse_table {
        //! SparseTable
        //! 冪等半群列にたいして区間(l,r] の結果を戻す
        //! 構築 O(NlogN) クエリO(1)
        //! min, max, gcd, lcm 等

        use std::ops::Range;

        /// 冪等半群
        pub trait Band {
            type T: Clone;
            fn operate(a: &Self::T, b: &Self::T) -> Self::T;
        }

        /// 最小値
        pub struct Min {}
        impl Band for Min {
            type T = i64;

            fn operate(a: &Self::T, b: &Self::T) -> Self::T {
                *a.min(b)
            }
        }

        /// SparseTable
        pub struct SparseTable<B: Band> {
            table: Vec<Vec<B::T>>,
        }

        impl<B: Band> SparseTable<B> {
            /// O(NlogN)
            pub fn new(v: &[B::T]) -> Self {
                let mut table = vec![v.to_vec()];

                for i in 1..64 - v.len().leading_zeros() as usize {
                    let mut tmp = vec![];
                    for j in 0..=v.len() - (1 << i) {
                        tmp.push(B::operate(
                            &table[i - 1][j],
                            &table[i - 1][j + (1 << (i - 1))],
                        ));
                    }
                    table.push(tmp);
                }

                SparseTable { table }
            }

            /// [l,r)
            /// O(1)
            pub fn fold(&self, range: Range<usize>) -> B::T {
                let i = 64 - (range.end - range.start).leading_zeros() as usize - 1;
                B::operate(
                    &self.table[i][range.start],
                    &self.table[i][range.end - (1 << i)],
                )
            }
        }
    }
}
