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
    input!(n1: usize, n2: usize, m: usize, ab: [(usize, usize); m]);

    let mut e = vec![vec![]; n1 + n2];
    for (a, b) in ab {
        e[a - 1].push(b - 1);
        e[b - 1].push(a - 1);
    }

    let ans = f(&e, 0) + f(&e, n1 + n2 - 1) + 1;

    println!("{}", ans);
}

fn f(e: &[Vec<usize>], index: usize) -> i64 {
    let mut q = std::collections::VecDeque::new();
    q.push_back((index, 0i64));
    let mut max = 0;
    let mut seen = std::collections::HashSet::new();
    while let Some(x) = q.pop_front() {
        if !seen.insert(x.0) {
            continue;
        }
        max = max.max(x.1);
        for i in &e[x.0] {
            if seen.contains(i) {
                continue;
            }
            q.push_back((*i, x.1 + 1));
        }
    }
    dbg!(max)
}
