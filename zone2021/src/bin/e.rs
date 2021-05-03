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
        r: usize,
        c: usize,
        a: [[i64; c - 1]; r],
        b: [[i64; c]; r - 1]
    );

    let mut e = vec![];
    for i in 0..r {
        for j in 0..c {
            let mut ee = vec![];
            let number = i * c + j;
            if j != 0 {
                ee.push((number - 1, a[i][j - 1]))
            }
            if j != c - 1 {
                ee.push((number + 1, a[i][j]))
            }

            if i != r - 1 {
                ee.push((number + r, b[i][j]))
            }
            if i != 0 {
                for k in 1..=i {
                    ee.push((number - (r * k), 1 + k as i64));
                }
            }
            e.push(ee);
        }
    }
    let ans = dijk::dijkstra(&e, 0, r * c - 1).unwrap();

    println!("{}", ans);
}

mod dijk {
    #[derive(Debug, Clone, Eq)]
    struct Node {
        pos: usize,
        cost: i64,
    }
    impl PartialEq for Node {
        fn eq(&self, other: &Node) -> bool {
            self.cost.eq(&other.cost)
        }
    }
    impl PartialOrd for Node {
        fn partial_cmp(&self, other: &Node) -> Option<std::cmp::Ordering> {
            Some(other.cost.cmp(&(self.cost)))
        }
    }
    impl Ord for Node {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            self.cost.cmp(&(other.cost))
        }
    }

    pub fn dijkstra(edge: &[Vec<(usize, i64)>], start: usize, end: usize) -> Option<i64> {
        let mut dist = vec![std::i64::MAX; edge.len()];
        let mut pq = std::collections::BinaryHeap::new();

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

        Some(dist[end])
    }
}

#[test]
fn utyuu() {
    let a = vec![
        "puvxlhwva",
        "fubxha",
        "tbxvtralb",
        "jnerjnerun",
        "xvzvgnpuvgb",
        "lhxban",
        "xnaxrvjb",
        "xvmhxhgnzr",
        "xbabubfuvav",
        "lnggrxvgn",
        "fnffbxhqntn",
        "lhxbab",
        "fuvehfuvgbfvgr",
        "chermragbjb",
        "bartnvfuvgnv",
        "ranwvqbevaxh",
        "mbar",
        "jb",
        "whaovfuvgr",
        "ubfuvvabqn",
        "xbabartnvjb",
        "xnanrgrxhereron",
        "bgbanfuvxh",
        "xbabubfuvjb",
        "ngbavfhehgfhzbevqn",
        "jnerjnerun",
        "nenfbvtbgbjb",
        "abmbznanv",
        "znrzhxvan",
        "urawvjb",
        "xvgnvfuvgrveh",
    ];

    let mut ans = vec![];
    for i in 0..a.len() {
        let mut aa = vec![];
        for j in a[i].to_string().chars() {
            let c = if (j as u8) < 110u8 {
                j as u8 + 13
            } else {
                j as u8 - 13
            };
            aa.push(c as char);
        }
        ans.push(aa);
    }
    for i in ans {
        for j in i {
            print!("{}", j);
        }
        println!();
    }
}

#[test]
fn story2() {
    input!(a: [i64; 400]);
    use competitive_library::algorithm::eratosthenes::Eratosthenes;

    let mut e = Eratosthenes::new();
    e.generate(1000);

    let set: std::collections::HashSet<_> = e.primes().iter().cloned().collect();
    let mut count = 0;
    for v in a {
        if set.contains(&(v as usize)) {
            count += 1;
        }
    }
    println!("{}", count);
}

#[test]
fn story3() {
    input!(n: usize, m: usize, ab: [(i64, i64); m]);
    let mut nn = vec![vec![]; n];
    for i in ab {
        nn[i.0 as usize].push(i.1);
        nn[i.1 as usize].push(i.0);
    }

    let mut max = 0;
    let mut ans = (0, 0, 0);
    for i in 0..n {
        for j in i + 1..n {
            for k in j + 1..n {
                let set: std::collections::HashSet<_> = nn[i]
                    .iter()
                    .chain(nn[j].iter().chain(nn[k].iter()))
                    .collect();
                if max < set.len() {
                    ans = (i, j, k);
                    max = set.len();
                }
            }
        }
    }
    println!("{} {} {}", ans.0, ans.1, ans.2);
}
