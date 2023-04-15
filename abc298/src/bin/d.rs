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
    let mut sc = io_pro::Scanner::new(std::io::BufReader::new(std::io::stdin().lock()));
    input!(sc = sc, q: usize);
    let mut vd = std::collections::VecDeque::new();
    vd.push_back(1);
    let mut ans = 1;
    let mut count_2 = 0;
    for _ in 0..q {
        input!(sc = sc, p: usize);
        if p == 1 {
            input!(sc = sc, x: i64);
            vd.push_back(x);
            ans *= 10;
            ans += x;
            ans %= 998244353;
        } else if p == 2 {
            let buff = vd[count_2];
            ans += 9982443530;
            ans -= buff * modpow(10, vd.len() as i64 - 1 - count_2 as i64, 998244353);
            ans %= 998244353;
            count_2 += 1;
        } else {
            println!("{}", ans % 998244353);
        }
    }
}

pub fn modpow(base: i64, exp: i64, n: i64) -> i64 {
    let (mut base, mut exp, n) = (base as u128, exp, n as u128);

    assert!(
        exp >= 0,
        "negative exponent cannot be used in modular exponentiation"
    );

    if exp == 0 {
        return 1;
    }

    let mut res = 1;
    base %= n;

    loop {
        if exp % 2 == 1 {
            res *= &base;
            res %= &n;
        }

        if exp == 1 {
            return res as i64;
        }

        exp /= 2;
        base *= base;
        base %= n;
    }
}
