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
    let mut sc = io_pro::Scanner::new(std::io::BufReader::new(std::io::stdin().lock()));

    input!(sc = sc, t: usize);

    for _ in 0..t {
        input!(
            sc = sc,
            n: usize,
            m: usize,
            c: [i64; n],
            uv: [(usize, usize); m]
        );

        let mut e = vec![vec![]; n];
        for &(u, v) in uv.iter() {
            e[u - 1].push((v - 1, 1));
            e[v - 1].push((u - 1, 1));
        }

        let a1 = dijk::dijkstra(&e, 0, n - 1, n);

        if a1.is_none() {
            println!("-1");
            continue;
        }
        let aa1 = a1.unwrap().1.iter().map(|x| c[*x]).collect::<Vec<_>>();
        let ans = f(&aa1, 0, 0, 0);
        println!("{}", ans.unwrap_or(-1));
    }
}

fn f(c: &[i64], index1: usize, index2: usize, count: i64) -> Option<i64> {
    if index1 == c.len() - 1 && index2 == c.len() - 1 {
        return Some(count);
    } else if count > c.len() as i64 * 3 {
        return None;
    }
    let mut buff = std::i64::MAX;

    if c[index1 + 1] != c[c.len() - 1 - (index2 + 1)] {
        buff == buff.min(f(c, index1 + 1, index2 + 1, count + 1).unwrap_or(std::i64::MAX));
    }
    if index2 != 0 && c[index1 + 1] != c[c.len() - 1 - (index2 - 1)] {
        buff == buff.min(f(c, index1 + 1, index2 - 1, count + 1).unwrap_or(std::i64::MAX));
    }
    if index1 != 0 && c[index1 - 1] != c[c.len() - 1 - (index2 + 1)] {
        buff == buff.min(f(c, index1 - 1, index2 + 1, count + 1).unwrap_or(std::i64::MAX));
    }

    if buff == std::i64::MAX {
        None
    } else {
        Some(buff)
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

            edge[position]
                .iter()
                .filter(|(to, c)| costs[*to].filter(|&d| d <= cost + c).is_none())
                .for_each(|&(to, c)| {
                    nodes.push(Node::new(to, cost + c, Some(position)));
                });
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

    #[cfg(test)]
    mod tests {
        use super::*;
        #[test]
        fn test_dijkstra() {
            let graph = vec![
                vec![(2, 10), (1, 1)],
                vec![(3, 2)],
                vec![(1, 1), (3, 3), (4, 1)],
                vec![(0, 7), (4, 2)],
                vec![],
            ];
            let l = graph.len();
            for (start, end, ans) in &[
                (0, 1, Some((1, vec![0, 1]))),
                (0, 3, Some((3, vec![0, 1, 3]))),
                (3, 0, Some((7, vec![3, 0]))),
                (0, 4, Some((5, vec![0, 1, 3, 4]))),
                (4, 0, None),
            ] {
                match dijkstra(&graph, *start, *end, l) {
                    Some((a, b)) => {
                        assert_eq!(a, ans.as_ref().unwrap().0);
                        assert_eq!(b, ans.as_ref().unwrap().1);
                    }
                    None => assert!(ans.is_none()),
                }
            }
        }
    }
}
