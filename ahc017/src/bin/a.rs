use std::{
    collections::{HashMap, HashSet},
    time::Instant,
};

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

fn main() {
    let since = Instant::now();
    let mut time = Instant::now().duration_since(since).as_millis();
    input!(
        n: usize,
        m: usize,
        d: usize,
        k: usize,
        mut uvw: [(usize, usize, i32); m]
    );
    let iii = from_vis::Input {
        D: d,
        K: k,
        M: m,
        es: uvw.clone(),
    };

    let mut xor = xorshift::XorShift::<u64>::new();

    let e = {
        let mut e = vec![vec![]; n];
        for (i, &(u, v, w)) in uvw.iter().enumerate() {
            e[u - 1].push((v - 1, w, i));
            e[v - 1].push((u - 1, w, i));
        }
        e
    };

    let vvvv = {
        let dj = dj(&e, &uvw, d, k, n);

        let mut vvv = (0..m).collect::<Vec<_>>();

        vvv.sort_by_key(|&x| std::cmp::Reverse(dj[x].0));
        let mut v = vec![std::collections::HashSet::new(); d];

        for (i, &value) in vvv.iter().enumerate() {
            v[value as usize % (d)].insert(i);
        }
        hukanou_nakusu(&mut v, &uvw, n, &mut xor, m, d, &e);
        v
    };
    if cfg!(debug_assertions) {
        dbg!(from_vis::compute_score(&iii, &to_vec(m, &vvvv)));
    }

    let saishoku = {
        let (color, max_color) = {
            let mut order = (0..m).collect::<Vec<_>>();

            let set = (0..m)
                .map(|x| {
                    e[uvw[x].0 - 1]
                        .iter()
                        .map(|x| x.2)
                        .chain(e[uvw[x].1 - 1].iter().map(|x| x.2))
                        .collect::<HashSet<_>>()
                })
                .collect::<Vec<_>>();

            order.sort_by_key(|x| std::cmp::Reverse(set[*x].len()));
            let mut color = vec![-1; m];

            let mut max = -1;
            for i in order {
                let c = set[i].iter().map(|x| color[*x]).collect::<HashSet<_>>();

                let min = (0..m as i32).find(|x| !c.contains(x)).unwrap();
                color[i] = min;

                max = max.max(min);
            }
            (color, dbg!(max))
        };
        let li = (max_color as usize + 1).min(d);
        let mut v = vec![std::collections::HashSet::new(); li];

        for (i, &value) in color.iter().enumerate() {
            v[value as usize % (li)].insert(i);
        }
        while v.len() < d {
            let mut max = 0;
            let mut index = 0;
            for i in 0..v.len() {
                max = v[i].len().max(max);
                if v[i].len() == max {
                    index = i;
                }
            }
            let mut a = std::collections::HashSet::new();
            let mut b = std::collections::HashSet::new();

            let mut c = v[index].iter().copied().collect::<Vec<_>>();
            c.sort_by_cached_key(|x| std::cmp::Reverse(uvw[*x].2));
            for i in 0..c.len() {
                if i % 2 == 0 {
                    a.insert(c[i]);
                } else {
                    b.insert(c[i]);
                }
            }

            v[index] = a;
            v.push(b);
        }

        dbg!(v.len());
        v
    };

    // if max_color as usize <= d && v.iter().all(|x| x.len() <= k) {
    //     // print_ans(&to_vec(m, &v));
    //     // return;
    //     if cfg!(debug_assertions) {
    //         dbg!(from_vis::compute_score(&iii, &to_vec(m, &v)));
    //     }
    // }

    //dbg!(from_vis::compute_score(&iii, &to_vec(m, &v)));

    let mut shokikai_set = {
        let mut shokikai_a = shokikai(&e, d, k, xor.next().unwrap() as usize % n, false);

        let since_b = Instant::now();
        let mut shokikai_a_score = dbg!(from_vis::compute_score(&iii, &to_vec(m, &shokikai_a)));
        let time_b = Instant::now().duration_since(since_b).as_millis();
        time = Instant::now().duration_since(since).as_millis();
        while time < 6000 - ((time_b * 12) / 10).max(1500).min(6000) {
            let shokikai_b = shokikai(&e, d, k, xor.next().unwrap() as usize % n, true);

            let shokikai_b_score = dbg!(from_vis::compute_score(&iii, &to_vec(m, &shokikai_b)));

            if shokikai_b_score < shokikai_a_score {
                shokikai_a = shokikai_b;
                shokikai_a_score = shokikai_b_score;
            }
            time = Instant::now().duration_since(since).as_millis();
        }
        shokikai_a
    };
    let mut all_iter = 0;
    time = Instant::now().duration_since(since).as_millis();
    while time < 5500 {
        all_iter += 1;
        if (all_iter & ((1 << 4) - 1)) == 0 {
            time = Instant::now().duration_since(since).as_millis();
        }

        let mut vv = (0..d).collect::<Vec<_>>();
        vv.sort_by_cached_key(|x| {
            std::cmp::Reverse(shokikai_set[*x].iter().map(|x| uvw[*x].2).sum::<i32>())
        });

        let a = *shokikai_set[vv[0]].iter().next().unwrap();

        shokikai_set[vv[0]].remove(&a);

        let mut b_index = xor.next().unwrap() as usize % d;
        while b_index == vv[0] || shokikai_set[b_index].len() >= k {
            b_index = xor.next().unwrap() as usize % d;
        }
        shokikai_set[b_index].insert(a);
    }

    dbg!(all_iter);
    let mut is_ok = false;
    while time < 5800 && !is_ok {
        all_iter += 1;
        if (all_iter & ((1 << 4) - 1)) == 0 {
            time = Instant::now().duration_since(since).as_millis();
        }
        is_ok = true;

        for i in 0..d {
            let g = inf_hantei(n, &shokikai_set[i], &uvw);
            if g.len() == 1 {
                continue;
            }
            is_ok = false;

            for (_, v) in g {
                let a = e[*v.iter().next().unwrap()][0].0;
                let mut a_index = 0;
                let mut b = xor.next().unwrap() as usize % m;

                for i in 0..d {
                    if shokikai_set[i].remove(&a) {
                        while !shokikai_set[i].insert(b) {
                            b = xor.next().unwrap() as usize % m;
                        }
                        a_index = i;
                        break;
                    }
                }

                for i in 0..d {
                    if i == a_index {
                        continue;
                    }
                    if shokikai_set[i].remove(&b) {
                        shokikai_set[i].insert(a);
                        break;
                    }
                }
            }
        }
    }

    hukanou_nakusu(&mut shokikai_set, &uvw, n, &mut xor, m, d, &e);

    dbg!(all_iter);

    if cfg!(debug_assertions) {
        dbg!(from_vis::compute_score(&iii, &to_vec(m, &shokikai_set)));
    }

    print_ans(&to_vec(m, &shokikai_set));
}

fn hukanou_nakusu(
    shokikai_set: &mut Vec<HashSet<usize>>,

    uvw: &[(usize, usize, i32)],
    n: usize,
    xor: &mut xorshift::XorShift<u64>,
    m: usize,
    d: usize,
    e: &[Vec<(usize, i32, usize)>],
) {
    let mut is_ok = false;
    while !is_ok {
        is_ok = true;

        for i in 0..d {
            let g = inf_hantei(n, &shokikai_set[i], uvw);
            if g.len() == 1 {
                continue;
            }
            is_ok = false;

            for (_, v) in g {
                let a = e[*v.iter().next().unwrap()][0].0;
                let mut a_index = 0;
                let mut b = xor.next().unwrap() as usize % m;

                for i in 0..d {
                    if shokikai_set[i].remove(&a) {
                        while !shokikai_set[i].insert(b) {
                            b = xor.next().unwrap() as usize % m;
                        }
                        a_index = i;
                        break;
                    }
                }

                for i in 0..d {
                    if i == a_index {
                        continue;
                    }
                    if shokikai_set[i].remove(&b) {
                        shokikai_set[i].insert(a);
                        break;
                    }
                }
            }
        }
    }
}

fn shokikai(
    e: &[Vec<(usize, i32, usize)>],
    d: usize,
    k: usize,
    start: usize,
    rev: bool,
) -> Vec<HashSet<usize>> {
    let dfs_array = {
        let mut array = vec![start];
        let mut seen = std::collections::HashSet::new();
        seen.insert(start);

        if rev {
            dfs_rev(&mut seen, &e, &mut array, start);
        } else {
            dfs(&mut seen, &e, &mut array, start);
        }
        array
    };
    let mut v = vec![std::collections::HashSet::new(); d];

    for (i, &value) in dfs_array.iter().enumerate() {
        v[i % d].insert(value);
    }
    v
}

fn to_vec(m: usize, kai: &[HashSet<usize>]) -> Vec<usize> {
    if cfg!(debug_assertions) {
        let mut set = std::collections::HashSet::new();
        for v in kai.iter() {
            for &i in v {
                set.insert(i);
            }
        }
        assert_eq!(set.len(), m);
    }
    let mut vv = vec![0; m];
    for (i, v) in kai.iter().enumerate() {
        for &j in v {
            vv[j] = i + 1;
        }
    }
    if cfg!(debug_assertions) {
        let mut set = std::collections::HashSet::new();
        for v in kai.iter() {
            for &i in v {
                set.insert(i);
            }
        }

        assert_eq!(set.len(), m);
        assert_eq!(vv.len(), m);
    }

    vv
}

#[proconio::fastout]
fn print_ans(v: &[usize]) {
    for value in v {
        print!("{} ", value);
    }
}

fn dfs(
    seen: &mut std::collections::HashSet<usize>,
    e: &[Vec<(usize, i32, usize)>],
    array: &mut Vec<usize>,
    node: usize,
) {
    for (v, w, index) in e[node].iter() {
        if seen.contains(index) {
            continue;
        }
        seen.insert(*index);
        array.push(*index);
        dfs(seen, e, array, *v);
    }
}
fn dfs_rev(
    seen: &mut std::collections::HashSet<usize>,
    e: &[Vec<(usize, i32, usize)>],
    array: &mut Vec<usize>,
    node: usize,
) {
    for (v, w, index) in e[node].iter().rev() {
        if seen.contains(&index) {
            continue;
        }
        seen.insert(*index);
        array.push(*index);
        dfs_rev(seen, e, array, *v);
    }
}

fn inf_hantei(
    n: usize,
    ban: &std::collections::HashSet<usize>,
    uvw: &[(usize, usize, i32)],
) -> HashMap<usize, HashSet<usize>> {
    let mut dsu = dsu::DisjointSetUnion::new(n);

    for (i, &(u, v, _)) in uvw.iter().enumerate() {
        if ban.contains(&i) {
            continue;
        }
        dsu.unite(u - 1, v - 1);
    }
    dsu.get_all_groups()
}

pub mod from_vis {
    #![allow(non_snake_case, unused_macros)]

    use std::collections::BinaryHeap;
    pub const INF: i32 = 1000000000;
    pub type Output = Vec<usize>;

    pub trait SetMinMax {
        fn setmin(&mut self, v: Self) -> bool;
        fn setmax(&mut self, v: Self) -> bool;
    }
    impl<T> SetMinMax for T
    where
        T: PartialOrd,
    {
        fn setmin(&mut self, v: T) -> bool {
            *self > v && {
                *self = v;
                true
            }
        }
        fn setmax(&mut self, v: T) -> bool {
            *self < v && {
                *self = v;
                true
            }
        }
    }

    #[derive(Clone, Debug)]
    pub struct Input {
        pub D: usize,
        pub K: usize,
        pub M: usize,
        pub es: Vec<(usize, usize, i32)>,
    }

    impl std::fmt::Display for Input {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            writeln!(f, "{} {} {} {}", self.M, self.es.len(), self.D, self.K)?;
            for &(u, v, w) in &self.es {
                writeln!(f, "{} {} {}", u + 1, v + 1, w)?;
            }

            Ok(())
        }
    }
    pub fn compute_score(input: &Input, out: &Output) -> i64 {
        let mut count = vec![0; input.D];
        for i in 0..input.es.len() {
            count[out[i] - 1] += 1;
        }
        for i in 0..input.D {
            if count[i] > input.K {
                panic!();
            }
        }
        let mut num = 0;
        let dist0 = compute_dist_matrix(input, out, !0);
        // let mut fs = vec![];
        for day in 1..=input.D {
            let dist = compute_dist_matrix(input, out, day);
            let mut tmp = 0;
            for i in 0..input.M {
                for j in i + 1..input.M {
                    tmp += (dist[i][j] - dist0[i][j]) as i64;
                }
            }
            num += tmp;
            //fs.push(tmp as f64 / (input.M * (input.M - 1) / 2) as f64);
        }
        let den = input.D * input.M * (input.M - 1) / 2;
        let avg = num as f64 / den as f64 * 1000.0;
        avg.round() as i64
    }

    fn compute_dist_matrix(input: &Input, out: &Output, day: usize) -> Vec<Vec<i32>> {
        let g = get_graph(input, out, day);
        let mut dist = vec![];
        for s in 0..input.M {
            dist.push(compute_dist(&g, s));
        }
        dist
    }
    fn compute_dist(g: &Vec<Vec<(usize, i32)>>, s: usize) -> Vec<i32> {
        let mut dist = vec![INF; g.len()];
        let mut que = BinaryHeap::new();
        que.push((0, s));
        dist[s] = 0;
        while let Some((d, u)) = que.pop() {
            let d = -d;
            if dist[u] != d {
                continue;
            }
            for &(v, w) in &g[u] {
                let d2 = d + w;
                if dist[v].setmin(d2) {
                    que.push((-d2, v));
                }
            }
        }
        dist
    }
    fn get_graph(input: &Input, out: &Output, day: usize) -> Vec<Vec<(usize, i32)>> {
        let mut g = vec![vec![]; input.M];
        for e in 0..input.es.len() {
            if out[e] != day {
                let (u, v, w) = input.es[e];
                g[u].push((v, w));
                g[v].push((u, w));
            }
        }
        g
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

fn dj(
    e: &[Vec<(usize, i32, usize)>],
    uvw: &[(usize, usize, i32)],
    d: usize,
    k: usize,
    n: usize,
) -> Vec<(i32, Vec<usize>)> {
    uvw.iter()
        .enumerate()
        .map(|(i, &(u, v, w))| {
            let mut ban = std::collections::HashSet::new();
            ban.insert(i);
            let a = dijkstra::dijkstra(&e, u - 1, v - 1, &ban, n).unwrap();
            (w - a.0, a.1)
        })
        .collect()
}

mod dijkstra {
    //! ダイクストラ
    use std::{cmp::Ordering, collections::BinaryHeap};

    #[derive(Debug, Clone, Eq)]
    pub struct Node {
        position: usize,
        cost: i32,
        from: Option<usize>,
    }
    impl Node {
        #[inline]
        pub fn new(position: usize, cost: i32, from: Option<usize>) -> Self {
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
        edge: &[Vec<(usize, i32, usize)>],
        start: usize,
        end: usize,
        ban: &std::collections::HashSet<usize>,
        vertex: usize,
    ) -> Option<(i32, Vec<usize>)> {
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
                .filter(|x| !ban.contains(&x.2))
                .filter(|(to, c, _)| costs[*to].filter(|&d| d <= cost + c).is_none())
                .for_each(|&(to, c, _)| {
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
