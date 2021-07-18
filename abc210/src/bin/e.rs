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
    input!(n: usize, m: usize, ac: [(i64, i64); m]);

    let mut ac = ac
        .iter()
        .map(|&(a, c)| (n as i64 % a, c))
        .collect::<Vec<_>>();
    ac.sort_by_key(|&(_, x)| x);
    let mut gcd = ac[0].0;
    let mut is_one = 0;
    for &(a, c) in ac.iter() {
        if a == 1 {
            is_one = c;
            break;
        }
        gcd = fn_gcd(gcd, a);
    }
    if gcd != 1 {
        println!("-1");
        return;
    }
    if is_one != 0 {
        println!("{}", is_one * (n as i64 - 1));
        return;
    }

    let mut ans = std::i64::MAX;
    for i in 0..m {
        if ac[i].1 * (n as i64 - 2) > ans {
            break;
        }
        for j in i + 1..m {
            if fn_gcd(ac[i].0, ac[j].0) != 1 {
                continue;
            }
            ans = ans.min(ac[i].1 * (n as i64 - ac[i].0) + ac[j].1 * (ac[i].0 - 1))
        }
    }

    println!("{}", ans);
}
pub fn fn_gcd(m: i64, n: i64) -> i64 {
    if m == 0 {
        n.abs()
    } else {
        fn_gcd(n % m, m)
    }
}
