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
    input!(n: usize, m: usize, ab: [(i64, i64); m]);
    let mut v = vec![vec![]; n];

    for (a, b) in ab {
        v[a as usize - 1].push(b as usize - 1);
    }
    let mut ans = 0;
    for i in 0..n {
        ans += dijk::dijkstra(&v, i);
    }

    println!("{}", ans);
}
mod dijk {
    //! ダイクストラ
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

    pub fn dijkstra(edge: &[Vec<usize>], start: usize) -> usize {
        let mut seen = std::collections::HashSet::new();
        let mut pq = std::collections::VecDeque::new();

        pq.push_back(start);
        seen.insert(start);

        while let Some(pos) = pq.pop_front() {
            seen.insert(pos);

            for &t in &edge[pos] {
                if seen.contains(&t) {
                    continue;
                }
                seen.insert(t);
                pq.push_back(t);
            }
        }
        seen.len()
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
