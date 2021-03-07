#[rustfmt::skip]
mod fast_input {
    #[macro_export] macro_rules! input{(sc=$sc:expr,$($r:tt)*)=>{input_inner!{$sc,$($r)*}};($($r:tt)*)=>{let mut sc=fast_input::Scanner::new(std::io::stdin().lock(),4096);input_inner!{sc,$($r)*}};}
    #[macro_export] macro_rules! input_inner{($sc:expr)=>{};($sc:expr,)=>{};($sc:expr,$var:ident:$t:tt$($r:tt)*)=>{let $var=read_value!($sc,$t);input_inner!{$sc $($r)*}};}
    #[macro_export] macro_rules! read_value{($sc:expr,($($t:tt),*))=>{($(read_value!($sc,$t)),*)};($sc:expr,[$t:tt;$len:expr])=>{(0..$len).map(|_|read_value!($sc,$t)).collect::<Vec<_>>()};($sc:expr,Chars)=>{read_value!($sc,String).chars().collect::<Vec<char>>()};($sc:expr,Usize1)=>{read_value!($sc,usize)-1};($sc:expr,$t:ty)=>{$sc.next::<$t>()};}
    pub struct Scanner {buf:Vec<u8>,pos: usize,}
    impl Scanner {
        pub fn new<R: std::io::Read>(mut reader: R, estimated: usize) -> Self {
            let mut buf = Vec::with_capacity(estimated);let _=std::io::copy(&mut reader,&mut buf).unwrap();if buf.last()!=Some(&b'\n'){panic!("{}", 0);}
            Scanner { buf, pos: 0 }
        }
        #[inline]
        pub fn next<T: std::str::FromStr>(&mut self) -> T where T::Err: std::fmt::Debug,{
            let mut start=None;loop{match(self.buf[self.pos],start.is_some()){(b' ',true)|(b'\n', true)|(b'\r', true)=>break,(_, true)|(b' ', false)|(b'\n',false)|(b'\r', false)=>self.pos+=1,(_, false)=>start=Some(self.pos),}}let target=&self.buf[start.unwrap()..self.pos];
            unsafe { std::str::from_utf8_unchecked(target) }.parse().unwrap()
        }
    }
}
#[proconio::fastout]
fn main() {
    input!(
        n: usize,
        m: usize,
        k: i64,
        t: [i64; n - 2],
        abcd: [(usize, usize, i64, i64); m]
    );

    let edge = {
        let mut v = vec![vec![]; n];
        for (a, b, t, k) in abcd.into_iter() {
            v[a - 1 as usize].push((b - 1, t, k));
            v[b - 1 as usize].push((a - 1, t, k));
        }
        v
    };
    let ans = di::dijkstra(&edge, &t, 0, n - 1).unwrap_or(std::i64::MAX);
    println!("{}", if k <= ans { -1 } else { ans });
}

mod di {
    #[derive(Debug, Clone, PartialEq, Eq, Ord)]
    struct Node {
        pos: usize,
        cost: i64,
    }

    impl PartialOrd for Node {
        fn partial_cmp(&self, other: &Node) -> Option<std::cmp::Ordering> {
            Some(other.cost.cmp(&(self.cost)))
        }
    }

    pub fn dijkstra(
        edge: &[Vec<(usize, i64, i64)>],
        t_norikae: &[i64],
        start: usize,
        end: usize,
    ) -> Option<i64> {
        let mut dist = vec![std::i64::MAX; edge.len()];
        let mut pq = std::collections::BinaryHeap::<Node>::new();

        pq.push(Node {
            pos: start,
            cost: 0,
        });
        dist[start] = 0;

        let mut ret = start == end;

        while let Some(Node { pos, cost }) = pq.pop() {
            if cost > dist[pos] {
                continue;
            }
            if ret {
                ret = false;
                dist[start] = std::i64::MAX;
            } else if end == pos {
                return Some(cost);
            }
            for (t, c, k) in &edge[pos] {
                let total_cost =
                    ((cost + if pos != 0 { t_norikae[pos - 1] } else { 0 } + *k - 1) / *k) * *k
                        + *c;
                if dist[*t] <= total_cost {
                    continue;
                }
                dist[*t] = total_cost;
                pq.push(Node {
                    pos: *t,
                    cost: total_cost,
                });
            }
        }
        None
    }

    #[test]
    fn test_dijkstra() {
        let graph = vec![
            vec![(2, 10), (1, 1)],
            vec![(3, 2)],
            vec![(1, 1), (3, 3), (4, 1)],
            vec![(0, 7), (4, 2)],
            vec![],
        ];

        assert_eq!(dijkstra(&graph, 0, 1), Some(1));
        assert_eq!(dijkstra(&graph, 0, 3), Some(3));
        assert_eq!(dijkstra(&graph, 3, 0), Some(7));
        assert_eq!(dijkstra(&graph, 0, 4), Some(5));
        assert_eq!(dijkstra(&graph, 4, 0), None);
    }
}
