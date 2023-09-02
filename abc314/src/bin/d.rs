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
    input!(sc = sc, n: usize, mut s: Chars, q: usize);
    let mut v = std::collections::VecDeque::new();
    let mut upper = None;
    for _ in 0..q {
        input!(sc = sc, t: usize);
        input!(sc = sc, x: usize, c: char);
        if t == 1 {
            v.push_back((x - 1, c));
        } else if t == 2 {
            while let Some(x) = v.pop_front() {
                s[x.0] = x.1;
            }
            upper = Some(false);
        } else {
            while let Some(x) = v.pop_front() {
                s[x.0] = x.1;
            }
            upper = Some(true);
        }
    }

    if let Some(x) = upper {
        if x {
            s = s
                .iter()
                .collect::<String>()
                .to_uppercase()
                .chars()
                .collect::<Vec<_>>();
        } else {
            s = s
                .iter()
                .collect::<String>()
                .to_lowercase()
                .chars()
                .collect::<Vec<_>>();
        }
    }
    while let Some(x) = v.pop_front() {
        s[x.0] = x.1;
    }
    println!("{}", s.iter().collect::<String>());
}
