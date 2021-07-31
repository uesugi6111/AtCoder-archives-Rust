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
    input!(n: usize, edge: [(usize, usize, i64); n - 1]);

    let mut e = vec![vec![]; n];
    for (a, b, c) in edge {
        e[a as usize].push((b, c));
        e[b as usize].push((a, c));
    }
    let out = std::io::stdout();
    let mut out = std::io::BufWriter::new(out.lock());
    let (cost, path) = td::tree_diamiter::<td::WeightedEdge>(&e);
    writeln!(&mut out, "{} {}", cost, path.len()).ok();
    for (i, v) in path.iter().enumerate() {
        if i == path.len() - 1 {
            writeln!(&mut out, "{}", v).ok();
        } else {
            write!(&mut out, "{} ", v).ok();
        }
    }
}

mod td {
    pub trait Edge {
        type T: Clone;
        fn get_edge(a: &Self::T) -> usize;
        fn get_cost(a: &Self::T) -> i64;
    }

    pub struct UnWeightedEdge {}
    impl Edge for UnWeightedEdge {
        type T = usize;
        #[inline]
        fn get_edge(a: &Self::T) -> usize {
            *a
        }
        #[inline]
        fn get_cost(_: &Self::T) -> i64 {
            1
        }
    }
    pub struct WeightedEdge {}
    impl Edge for WeightedEdge {
        type T = (usize, i64);

        #[inline]
        fn get_edge(a: &Self::T) -> usize {
            a.0
        }
        #[inline]
        fn get_cost(a: &Self::T) -> i64 {
            a.1
        }
    }

    pub fn tree_diamiter<E: Edge>(e: &[Vec<E::T>]) -> (i64, Vec<usize>) {
        let (_, path_1) = bfs::<E>(e, 0);
        bfs::<E>(e, path_1[path_1.len() - 1])
    }

    #[inline]
    fn bfs<E: Edge>(e: &[Vec<E::T>], start: usize) -> (i64, Vec<usize>) {
        let mut que = std::collections::VecDeque::new();
        let mut max_cost_index = (start, 0);
        let mut previous = vec![None; e.len()];
        previous[start] = Some(start);

        que.push_back((start, 0));

        while let Some((v, cost)) = que.pop_front() {
            for edge in e[v].iter() {
                let to = E::get_edge(edge);
                if previous[to].is_some() {
                    continue;
                }
                previous[to] = Some(v);
                que.push_back((E::get_edge(edge), cost + E::get_cost(edge)));
            }

            if max_cost_index.1 < cost {
                max_cost_index = (v, cost);
            }
        }
        (max_cost_index.1, restore_path(max_cost_index.0, &previous))
    }

    #[inline]
    fn restore_path(end: usize, previous: &[Option<usize>]) -> Vec<usize> {
        let mut buff = end;
        let mut v = vec![buff];

        while let Some(i) = previous[buff] {
            if buff == i {
                break;
            }
            buff = i;
            v.push(buff);
        }
        v.reverse();
        v
    }
}
