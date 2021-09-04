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
    input!(n: usize, k: i64, a: [i64; n]);
    let mut a = a;
    a.sort();

    let ans = binary_search(&a, k);

    let mut sum = 0_i64;
    let mut count = 0;
    for &i in a.iter() {
        sum += (ans + 1..i + 1).sum::<i64>();
        count = 0.max(i - (ans - 1));
    }
    sum += (k - count) * ans;
    println!("{}", sum);
}

fn binary_search(v: &[i64], n: i64) -> i64 {
    let mut left = -1;
    let mut right = v[v.len() - 1];

    while right - left > 1 {
        let mid = left + (right - left) / 2;

        if is_ok(v, mid, n) {
            right = mid;
        } else {
            left = mid;
        };
    }

    right
}

fn is_ok(v: &[i64], mid: i64, n: i64) -> bool {
    let mut sum = 0;
    for i in v.iter() {
        sum += 0.max(i - mid);
    }
    sum <= n
}
