#[rustfmt::skip]
mod io_pro {
    #[macro_export] macro_rules! input{(sc=$sc:expr,$($r:tt)*)=>{input_inner!{$sc,$($r)*}};($($r:tt)*)=>{let mut sc=io_pro::Scanner::new(std::io::stdin().lock());input_inner!{sc,$($r)*}};}
    #[macro_export] macro_rules! input_inner{($sc:expr)=>{};($sc:expr,)=>{};($sc:expr,$var:ident:$t:tt$($r:tt)*)=>{let $var=read_value!($sc,$t);input_inner!{$sc $($r)*}};($sc:expr,mut $var:ident:$t:tt$($r:tt)*)=>{let mut $var=read_value!($sc,$t);input_inner!{$sc $($r)*}};}
    #[macro_export] macro_rules! read_value{($sc:expr,($($t:tt),*))=>{($(read_value!($sc,$t)),*)};($sc:expr,[$t:tt;$len:expr])=>{(0..$len).map(|_|read_value!($sc,$t)).collect::<Vec<_>>()};($sc:expr,Chars)=>{read_value!($sc,String).chars().collect::<Vec<char>>()};($sc:expr,Usize1)=>{read_value!($sc,usize)-1};($sc:expr,$t:ty)=>{$sc.next::<$t>()};}
    pub struct Scanner{s:Box<str>,input:std::iter::Peekable<std::str::SplitAsciiWhitespace<'static>>,}
    impl Scanner{
        pub fn new<R:std::io::Read>(mut reader:R)->Self{let mut sc=Scanner{s:{let mut s=String::new();reader.read_to_string(&mut s).unwrap();s.into_boxed_str()},input:"".split_ascii_whitespace().peekable(),};let s:&'static str=unsafe{std::mem::transmute(&*sc.s)};sc.input=s.split_ascii_whitespace().peekable();sc}
        #[inline]pub fn next<T:std::str::FromStr>(&mut self)->T where T::Err:std::fmt::Debug,{self.input.next().unwrap().parse::<T>().expect("Parse error")}
    }
}
#[proconio::fastout]
fn main() {
    input!(n: i64);

    let bs = BinarySearch::new(n, -1, 1000000);
    let a = bs.search(|&x, j| j.pow(3) < x);

    let mut b_set = std::collections::BTreeSet::new();

    for i in 0..10000 {
        b_set.insert(ff(a, i));
    }
    for i in 0..10000 {
        b_set.insert(ff(a - 1, i));
    }

    println!("{}", b_set.range(n..).next().unwrap_or(&0));
}

fn ff(a: i64, b: i64) -> i64 {
    a.pow(3) + a.pow(2) * (b) + a * (b).pow(2) + (b).pow(3)
}
pub struct BinarySearch<T> {
    target: T,
    min: i64,
    max: i64,
}
impl<T> BinarySearch<T> {
    pub fn new(target: T, min: i64, max: i64) -> Self {
        Self { target, min, max }
    }
    /// f が true を帰す最小値を探す
    pub fn search<F>(&self, f: F) -> i64
    where
        F: Fn(&T, i64) -> bool,
    {
        let mut left = self.min;
        let mut right = self.max;

        while right - left > 1 {
            let mid = left + (right - left) / 2;

            if f(&self.target, mid) {
                left = mid;
            } else {
                right = mid
            };
        }
        right
    }
}
