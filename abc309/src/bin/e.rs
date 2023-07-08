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
    input!(n: usize, m: usize, p: [usize; n - 1], xy: [(usize, i64); m]);

    let mut ko = vec![vec![]; n];
    for i in 1..n {
        ko[p[i - 1] - 1].push(i);
    }
    let mut z = std::collections::HashMap::new();
    for (x, y) in xy {
        let buff = y.max(*z.get(&(x - 1)).unwrap_or(&(-1)));
        z.insert(x - 1, buff);
    }

    let mut ans = vec![-1i64; n];

    let mut q = std::collections::VecDeque::new();
    q.push_back(0usize);

    let mut seen = std::collections::HashSet::new();
    while let Some(x) = q.pop_front() {
        ans[x] = ans[x].max(*z.get(&x).unwrap_or(&(-1)));
        for i in &ko[x] {
            if !seen.insert(i) {
                continue;
            }
            ans[*i] = ans[x] - 1;
            q.push_back(*i);
        }
    }

    println!("{}", ans.iter().filter(|&x| *x >= 0).count());
}
