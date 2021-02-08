#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[proconio::fastout]
fn main() {
    input!(n: usize, m: usize, abc: [(usize, usize, i64); m]);

    let edge = {
        let mut v = vec![vec![]; n];
        for (a, b, c) in abc.into_iter() {
            v[a - 1 as usize].push((b - 1, c));
        }
        v
    };

    for i in (0..n).map(|x| dijkstra::dijkstra(&edge, x, x).unwrap_or(-1)) {
        println!("{}", i);
    }
}

mod dijkstra {
    #[derive(Debug, Clone, PartialEq, Eq, Ord)]
    struct Node {
        pos: usize,
        cost: i64,
    }

    impl PartialOrd for Node {
        fn partial_cmp(&self, other: &Node) -> Option<std::cmp::Ordering> {
            Some(other.cmp(self))
        }
    }

    pub fn dijkstra(edge: &[Vec<(usize, i64)>], start: usize, end: usize) -> Option<i64> {
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
            }
            for (t, c) in &edge[pos] {
                let total_cost = cost + *c;
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

        if dist[end] == std::i64::MAX {
            None
        } else {
            Some(dist[end])
        }
    }
}
