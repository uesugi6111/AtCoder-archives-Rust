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
    input!(n: usize, m: usize, h: [i64; n], uv: [(usize, usize); m]);

    let mut e = vec![vec![]; n];
    for (u, v) in uv {
        let (c1, c2) = if h[u - 1] < h[v - 1] {
            (-(h[v - 1] - h[u - 1]), 2 * (h[v - 1] - h[u - 1]))
        } else {
            (2 * (h[u - 1] - h[v - 1]), -(h[u - 1] - h[v - 1]))
        };
        e[u - 1].push((v - 1, c2));
        e[v - 1].push((u - 1, c1));
    }

    let ans = spfa::spfa(&e, 0).unwrap();

    println!("{}", -ans.iter().min().unwrap());
}

mod spfa {
    //! SPFA

    use std::collections::VecDeque;

    pub fn spfa(edge: &[Vec<(usize, i64)>], start: usize) -> Option<Vec<i64>> {
        let mut pending = vec![false; edge.len()];
        let mut times = vec![0; edge.len()];
        let mut costs = vec![std::i64::MAX; edge.len()];
        let mut q = VecDeque::new();
        q.push_back(start);
        times[start] = 1;
        costs[start] = 0;
        pending[start] = true;

        while let Some(p) = q.pop_front() {
            pending[p] = false;

            for &(to, c) in &edge[p] {
                let cost = costs[p] + c;
                if costs[to] <= cost {
                    continue;
                }
                costs[to] = cost;
                if !pending[to] {
                    times[to] += 1;
                    if times[to] >= edge.len() {
                        return None;
                    }
                    pending[to] = true;
                    q.push_back(to);
                }
            }
        }

        Some(costs)
    }

    #[cfg(test)]
    mod tests {
        use super::spfa;

        #[test]
        fn test_spfa() {
            let graph = vec![
                vec![(2, 10), (1, 1)],
                vec![(3, 2)],
                vec![(1, 1), (3, 3), (4, 1)],
                vec![(0, 7), (4, 2)],
                vec![],
            ];
            let ans = spfa(&graph, 0);

            assert_eq!(ans.unwrap(), vec![0, 1, 10, 3, 5]);
        }
    }
}
