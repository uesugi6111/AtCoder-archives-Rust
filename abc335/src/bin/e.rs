use std::collections::{BTreeMap, HashMap, HashSet};

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
    input!(n: usize, m: usize, a: [i64; n], uv: [(usize, usize); m]);

    let mut e = vec![vec![]; n];

    for (u, v) in uv {
        if a[u - 1] <= a[v - 1] {
            e[u - 1].push(v - 1);
        }
        if a[v - 1] <= a[u - 1] {
            e[v - 1].push(u - 1);
        }
    }
    let mut map = BTreeMap::new();
    let mut set = HashSet::new();
    let mut memo = HashMap::new();
    let ans = f(&e, 0, &mut map, n - 1, &a, &mut set, &mut memo);
    println!("{}", ans);
}

fn f(
    e: &[Vec<usize>],
    now: usize,
    map: &mut BTreeMap<i64, i64>,
    n: usize,
    a: &[i64],
    set: &mut HashSet<usize>,
    memo: &mut HashMap<i64, i64>,
) -> i64 {
    if let Some(&x) = memo.get(&a[now]) {
        return x;
    }
    if set.contains(&now) {
        return 0;
    }
    set.insert(now);
    if *map.last_key_value().unwrap_or((&0, &0)).0 > a[now] {
        return 0;
    }
    *map.entry(a[now]).or_default() += 1;
    let mut max = 0;

    for i in &e[now] {
        if n == *i {
            max = max.max(map.len() as i64 + if map.contains_key(&a[*i]) { 0 } else { 1 });
        } else {
            max = max.max(f(e, *i, map, n, a, set, memo));
        }
    }
    *map.entry(a[now]).or_default() -= 1;
    if *map.get(&a[now]).unwrap() == 0 {
        map.remove(&a[now]);
    }
    set.remove(&now);
    memo.insert(a[now], max);
    max
}
