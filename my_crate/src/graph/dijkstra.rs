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

    while let Some(Node { pos, cost }) = pq.pop() {
        if pos == end {
            return Some(cost);
        }
        if cost > dist[pos] {
            continue;
        }

        for (t, c) in &edge[pos] {
            if dist[*t] <= cost + c {
                continue;
            }
            dist[*t] = cost + c;
            pq.push(Node {
                pos: *t,
                cost: cost + c,
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
