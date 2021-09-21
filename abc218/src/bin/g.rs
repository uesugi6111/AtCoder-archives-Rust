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
fn main() {
    input!(n: usize, m: usize, s: usize, abc: [(usize, usize, i64); m]);

    let mut e = vec![vec![]; n];

    for (from, to, cost) in abc {
        e[from].push((to, cost));
    }
    let ans = directed_mst::directed_mst(&e, s).unwrap();
    use std::io::{stdout, BufWriter, Write};
    let out = stdout();
    let mut out_lock = BufWriter::new(out.lock());
    writeln!(&mut out_lock, "{}", ans.0).ok();
    for (i, c) in ans.1.iter().enumerate() {
        if i == ans.1.len() - 1 {
            writeln!(&mut out_lock, "{}", c).ok();
        } else {
            write!(&mut out_lock, "{} ", c).ok();
        }
    }
}

mod directed_mst {
    use std::cmp::Ordering;

    use disjoint_set_union_undo::DisjointSetUnionRollback;
    use skew_heap_lazy::SkewHeap;

    #[derive(Debug, Clone, PartialEq, Eq, PartialOrd)]
    struct Edge {
        from: usize,
        to: usize,
        cost: i64,
    }

    impl Ord for Edge {
        fn cmp(&self, other: &Self) -> Ordering {
            self.cost.cmp(&(other.cost))
        }
    }
    pub fn directed_mst(e: &[Vec<(usize, i64)>], root: usize) -> Option<(i64, Vec<usize>)> {
        let mut uf_undo = DisjointSetUnionRollback::new(e.len());
        let mut from_v = vec![(0, None); e.len()];
        let mut from_cost = vec![0; e.len()];
        let mut used = vec![0; e.len()];
        let mut heap = vec![SkewHeap::new(); e.len()];
        let mut cycles = vec![];
        used[root] = 2;

        for i in 0..e.len() {
            for j in 0..e[i].len() {
                heap[e[i][j].0].push(
                    e[i][j].1,
                    Edge {
                        from: i,
                        to: e[i][j].0,
                        cost: e[i][j].1,
                    },
                );
            }
        }

        let mut ans = 0;
        for start in 0..e.len() {
            if used[start] != 0 {
                continue;
            }
            let mut current = start;
            let mut processing = vec![];
            while used[current] != 2 {
                used[current] = 1;
                processing.push(current);

                if let Some((c, e)) = heap[current].pop() {
                    from_v[current] = (uf_undo.root(e.from), Some(e));
                    from_cost[current] = c;
                } else {
                    return None;
                }
                if from_v[current].0 == current {
                    continue;
                }
                ans += from_cost[current];

                if used[from_v[current].0] != 1 {
                    current = from_v[current].0;
                    continue;
                }
                let mut p = current;
                let time = uf_undo.get_history_length();
                while {
                    if !heap[p].is_empty() {
                        heap[p].add(-from_cost[p]);
                    }
                    uf_undo.unite(p, current);
                    let buff = heap[p].node.take();
                    SkewHeap::merge(&mut heap[current].node, buff);

                    p = uf_undo.root(from_v[p].0);
                    p != current
                } {}
                cycles.push((from_v[p].clone(), time));
            }
            for v in processing {
                used[v] = 2;
            }
        }
        for (f, time) in cycles
            .iter()
            .rev()
            .take(0.max(cycles.len() as i64 - 1) as usize)
        {
            let vrepr = uf_undo.root(f.1.as_ref().unwrap().to);
            uf_undo.rollback(*time);
            let vinc = uf_undo.root(from_v[vrepr].1.as_ref().unwrap().to);

            from_v[vinc] = from_v[vrepr].clone();
            from_v[vrepr] = f.to_owned();
        }
        let mut edges = vec![11111; e.len()];
        for i in 0..e.len() {
            edges[i] = if i == root {
                root
            } else {
                from_v[i].1.as_ref().unwrap().from
            };
        }

        Some((ans, edges))
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        #[test]
        fn test_directed_mst() {
            let input = vec![(0, 1, 10), (0, 2, 10), (0, 3, 3), (3, 2, 4)];
            let mut e = vec![vec![]; 4];

            for (from, to, cost) in input {
                e[from].push((to, cost));
            }
            let ans = directed_mst(&e, 0).unwrap();
            assert_eq!(ans.0, 17);
            assert_eq!(ans.1, &[0, 0, 3, 0]);
        }
        #[test]
        fn test_directed_mst2() {
            let input = vec![
                (3, 1, 10),
                (1, 2, 1),
                (2, 0, 1),
                (0, 1, 1),
                (2, 6, 10),
                (6, 4, 1),
                (4, 5, 1),
                (5, 6, 1),
            ];

            let mut e = vec![vec![]; 7];

            for (from, to, cost) in input {
                e[from].push((to, cost));
            }
            let ans = directed_mst(&e, 3).unwrap();
            assert_eq!(ans.0, 24);
            assert_eq!(ans.1, &[2, 3, 1, 3, 6, 4, 2]);
        }
    }

    mod disjoint_set_union_undo {
        //! Union find undo
        use std::collections::{HashMap, HashSet, VecDeque};
        #[derive(Debug, Clone)]
        enum Node {
            Root(usize, usize),
            Child(usize),
        }
        /// UnionFind
        /// 経路圧縮を行わないことで undo を可能にする
        #[derive(Clone, Debug)]
        pub struct DisjointSetUnionRollback {
            uf: Vec<Node>,
            history: VecDeque<(usize, Node)>,
            restore_point: Option<usize>,
        }

        impl DisjointSetUnionRollback {
            /// 要素数 n の dsu を構築する
            #[inline]
            pub fn new(n: usize) -> DisjointSetUnionRollback {
                DisjointSetUnionRollback {
                    uf: vec![Node::Root(1, 1); n],
                    history: VecDeque::new(),
                    restore_point: None,
                }
            }

            /// 根を取得
            /// 経路圧縮を行わない
            #[inline]
            pub fn root(&self, target: usize) -> usize {
                match self.uf[target] {
                    Node::Root(_, _) => target,
                    Node::Child(par) => self.root(par),
                }
            }

            /// 対象の木をマージ
            /// 経路圧縮を行わないため変更されるノード数は高々2
            /// 変更箇所をスタックで保存
            #[inline]
            pub fn unite(&mut self, x: usize, y: usize) -> bool {
                let rx = self.root(x);
                let ry = self.root(y);
                if rx == ry {
                    return false;
                }
                self.history.push_back((rx, self.uf[rx].clone()));
                self.history.push_back((ry, self.uf[ry].clone()));
                let size_x = self.size(rx);
                let size_y = self.size(ry);
                let rank_x = self.rank(rx);
                let rank_y = self.rank(ry);
                let (i, j) = if rank_x > rank_y { (rx, ry) } else { (ry, rx) };
                self.uf[i] = Node::Root(
                    size_x + size_y,
                    (rank_x.min(rank_y) + 1).max(rank_x.max(rank_y)),
                );
                self.uf[j] = Node::Child(i);

                true
            }

            /// 同じ木に存在するか
            #[inline]
            pub fn is_same(&mut self, x: usize, y: usize) -> bool {
                self.root(x) == self.root(y)
            }

            /// 所属する木のサイズ
            pub fn size(&mut self, x: usize) -> usize {
                let root = self.root(x);
                match self.uf[root] {
                    Node::Root(size, _) => size,
                    Node::Child(_) => 1,
                }
            }
            /// 所属する木のランク
            #[inline]
            pub fn rank(&mut self, x: usize) -> usize {
                let root = self.root(x);
                match self.uf[root] {
                    Node::Root(_, rank) => rank,
                    Node::Child(_) => 1,
                }
            }

            /// unite 操作の undo
            #[inline]
            pub fn undo(&mut self) {
                for _ in 0..2 {
                    let (index, node) = self.history.pop_back().unwrap();
                    self.uf[index] = node;
                }
            }

            /// 現時点の状態を保存
            /// 復元には rollback_snapshot
            #[inline]
            pub fn snapshot(&mut self) {
                self.restore_point = Some(self.history.len() >> 1);
            }

            /// 現時点での保存されている操作回数を返す
            #[inline]
            pub fn get_history_length(&self) -> usize {
                self.history.len() >> 1
            }

            /// rollback_snapshot で保存された状態へ復元
            #[inline]
            pub fn rollback_snapshot(&mut self) {
                self.rollback(self.restore_point.unwrap());
            }

            /// 復元
            /// 任意のタイミングで get_history_length を実行し取得した 値を使用する
            #[inline]
            pub fn rollback(&mut self, n: usize) {
                assert!(self.history.len() >= n << 1);

                while self.history.len() > n << 1 {
                    self.undo();
                }
            }

            /// 同じ木に含まれるノードを返す
            #[inline]
            pub fn get_same_group(&mut self, x: usize) -> HashSet<usize> {
                let root = self.root(x);
                let mut g = HashSet::new();
                for i in 0..self.uf.len() {
                    if root == self.root(i) {
                        g.insert(i);
                    }
                }
                g
            }

            /// 全ノードを返却
            #[inline]
            pub fn get_all_groups(&mut self) -> HashMap<usize, HashSet<usize>> {
                let mut map: HashMap<usize, HashSet<usize>> = HashMap::new();
                for i in 0..self.uf.len() {
                    let root = self.root(i);

                    map.entry(root).or_insert_with(HashSet::new).insert(i);
                }
                map
            }
        }

        #[cfg(test)]
        mod tests {
            use super::*;

            #[test]
            fn test_dsu_rollback() {
                let mut dsu = DisjointSetUnionRollback::new(6);

                dsu.unite(0, 1);
                assert!(dsu.is_same(0, 1));
                dsu.unite(1, 2);
                assert!(dsu.is_same(0, 2));
                assert_eq!(dsu.size(0), 3);
                assert!(!dsu.is_same(0, 3));
                dsu.snapshot();
                dsu.unite(0, 3);
                dsu.unite(3, 4);
                dsu.unite(4, 5);
                assert_eq!(dsu.size(5), 6);
                assert!(dsu.is_same(0, 5));
                dsu.undo();
                assert!(!dsu.is_same(0, 5));
                dsu.rollback_snapshot();
                assert!(dsu.is_same(0, 2));
                assert_eq!(dsu.size(0), 3);
                assert!(!dsu.is_same(0, 3));
                dsu.rollback(0);
                assert!(!dsu.is_same(0, 1));
                assert_eq!(dsu.get_history_length(), 0);
            }
        }
    }

    mod skew_heap_lazy {
        //! Skew Heap Lazy

        use std::mem::swap;
        #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
        pub struct Heap<T: Clone> {
            pub cost: i64,
            pub value: T,
            pub lazy: Option<i64>,
            pub left: Option<Box<Heap<T>>>,
            pub right: Option<Box<Heap<T>>>,
        }

        impl<T: Clone> Heap<T> {
            pub fn new(cost: i64, value: T) -> Option<Box<Heap<T>>> {
                Some(Box::new(Heap {
                    cost,
                    value,
                    lazy: None,
                    left: None,
                    right: None,
                }))
            }
        }
        #[derive(Default, Clone)]
        pub struct SkewHeap<T: Clone> {
            pub node: Option<Box<Heap<T>>>,
        }
        impl<T: Clone> SkewHeap<T> {
            #[inline]
            pub fn new() -> Self {
                Self { node: None }
            }
            #[inline]
            pub fn from(cost: i64, value: T) -> Self {
                let mut s = Self { node: None };
                s.push(cost, value);
                s
            }

            #[inline]
            pub fn push(&mut self, cost: i64, value: T) {
                SkewHeap::merge(&mut self.node, Heap::new(cost, value));
            }
            #[inline]
            pub fn top(&self) -> Option<(i64, T)> {
                Some((self.node.as_ref()?.cost, self.node.as_ref()?.value.clone()))
            }
            #[inline]
            pub fn pop(&mut self) -> Option<(i64, T)> {
                Self::propagate(&mut self.node);
                let value = self.top()?;

                let (mut left, right) = {
                    let mut tmp = self.node.take().unwrap();
                    (tmp.left.take(), tmp.right.take())
                };
                SkewHeap::merge(&mut left, right);
                swap(&mut self.node, &mut left);

                Some(value)
            }

            #[inline]
            pub fn is_empty(&self) -> bool {
                self.node.is_none()
            }

            #[inline]
            pub fn merge(a: &mut Option<Box<Heap<T>>>, mut b: Option<Box<Heap<T>>>) {
                if a.is_none() {
                    swap(a, &mut b);
                    return;
                }
                if b.is_none() {
                    return;
                }
                Self::propagate(a);
                Self::propagate(&mut b);

                if a.as_ref().unwrap().cost > b.as_ref().unwrap().cost {
                    swap(a, &mut b);
                }
                SkewHeap::merge(&mut a.as_mut().unwrap().right, b);

                let tmp = a.as_mut().unwrap();
                swap(&mut tmp.left, &mut tmp.right);
            }

            #[inline]
            pub fn add(&mut self, value: i64) {
                self.node.as_mut().unwrap().lazy = Some(value);
                Self::propagate(&mut self.node);
            }
            #[inline]
            fn propagate(node: &mut Option<Box<Heap<T>>>) {
                if let Some(n) = node.as_mut() {
                    if n.lazy.is_none() {
                        return;
                    }
                    if let Some(l) = n.left.as_mut() {
                        l.lazy = n.lazy;
                    }
                    if let Some(r) = n.right.as_mut() {
                        r.lazy = n.lazy;
                    }

                    n.cost += n.lazy.unwrap();
                    n.lazy = None;
                }
            }
        }

        #[cfg(test)]
        mod tests {
            use super::*;
            #[test]
            fn test_heap() {
                let mut a = vec![SkewHeap::new(); 5];

                for i in 0..30 {
                    a[i % 5].push(i as i64, 0);
                }

                for (i, e) in a.iter().enumerate() {
                    assert_eq!(e.top().unwrap().0, i as i64);
                }

                for i in 1..5 {
                    let buff = a[i].node.take();
                    SkewHeap::merge(&mut a[0].node, buff);
                }

                for i in 0..15 {
                    assert_eq!(a[0].pop().unwrap().0, i);
                }
                a[0].add(5);
                for i in 15..30 {
                    assert_eq!(a[0].pop().unwrap().0, i + 5);
                }
            }
        }
    }
}
