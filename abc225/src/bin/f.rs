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
    input!(n: usize, k: usize, s: [String; n]);
    let mut map = std::collections::BTreeMap::new();

    for i in 0..n {
        map.entry(s[i].clone()).or_insert_with(|| vec![]).push(i);
    }
    let mut s_map = std::collections::BTreeMap::new();

    let (pre, _) = map.iter().next().unwrap();

    for (key, c) in map.iter() {
        if !key.starts_with(pre) {
            break;
        }
        for &l in c.iter() {
            let mut set = std::collections::BTreeSet::new();
            set.insert(l);
            s_map.insert(key.clone(), set);
        }
    }
    for i in 0..k - 1 {
        let mut next_map = std::collections::BTreeMap::new();
        for (key, c) in s_map.iter().take(900) {
            if !key.starts_with(pre) {
                break;
            }
            for (m_k, m_v) in map.iter() {
                for &j in m_v.iter() {
                    if c.contains(&j) {
                        continue;
                    }
                    let mut set = c.clone();
                    set.insert(j);
                    next_map.insert(format!("{}{}", key, m_k), set);
                }
            }
        }
        s_map = next_map;
    }
    println!("{}", s_map.iter().next().unwrap().0);
}
