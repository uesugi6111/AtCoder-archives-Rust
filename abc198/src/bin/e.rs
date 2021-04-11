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
    input!(n: usize, c: [i64; n], ab: [(usize, usize); n - 1]);

    let g = {
        let mut g = vec![vec![]; n + 1];
        for (a, b) in ab {
            g[a].push(b);
            g[b].push(a);
        }
        g
    };

    let mut color = vec![None; n + 1];
    color[1] = Some({
        let mut set = std::collections::BTreeSet::new();
        set.insert(c[0]);
        set
    });

    let mut que = std::collections::VecDeque::new();

    // 次数が0の点をキューに入れる

    que.push_back(1);
    let mut ans = vec![];
    ans.push(1);
    while !que.is_empty() {
        // 幅優先探索
        let now = que.pop_front().unwrap();

        for &e in g[now].iter() {
            if color[e].is_some() {
                continue;
            }
            let ccc = c[e - 1];
            if !color[now].as_ref().unwrap().contains(&ccc) {
                ans.push(e);
            }
            let mut set = color[now].as_ref().unwrap().clone();
            set.insert(ccc);
            color[e] = Some(set);

            que.push_back(e);
        }
    }
    ans.sort();
    for v in ans {
        println!("{}", v);
    }
}

/* topo_sort(G): グラフG をトポロジカルソート
   返り値: トポロジカルソートされた頂点番号
   計算量: O(|E|+|V|)
*/
fn topo_sort(g: &[Vec<usize>]) -> Vec<usize> {
    // bfs
    let mut ans = vec![];
    let n = g.len();
    // ind[i]: 頂点iに入る辺の数(次数)
    let mut ind = vec![0; n];
    // 次数を数えておく
    for i in 0..n {
        for &e in g[i].iter() {
            ind[e] += 1;
        }
    }
    let mut que = std::collections::VecDeque::new();

    // 次数が0の点をキューに入れる
    for i in 0..n {
        if ind[i] == 0 {
            que.push_back(i);
        }
    }
    while !que.is_empty() {
        // 幅優先探索
        let now = que.pop_front().unwrap();

        ans.push(now);

        for &e in g[now].iter() {
            ind[e] -= 1;
            if ind[e] == 0 {
                que.push_back(e);
            }
        }
    }
    ans
}
