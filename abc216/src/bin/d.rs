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
    let mut sc = io_pro::Scanner::new(std::io::stdin().lock());
    input!(sc = sc, n: usize, m: usize);

    let mut v = vec![];
    let mut map = std::collections::HashMap::<i64, usize>::new();
    let mut q = std::collections::VecDeque::new();
    for i in 0..m {
        input!(sc = sc, k: usize, a: [i64; k]);
        if map.contains_key(&a[0]) {
            q.push_back((map.get(&a[0]).unwrap().clone(), i));
        } else {
            map.insert(a[0], i);
        }

        v.push(std::collections::VecDeque::from(a));
    }

    while let Some(a) = q.pop_front() {
        v[a.0].pop_front();

        if v[a.0].front().is_some() {
            if map.contains_key(&v[a.0].front().unwrap()) {
                q.push_back((map.get(&v[a.0].front().unwrap()).unwrap().clone(), a.0));
            } else {
                map.insert(*v[a.0].front().unwrap(), a.0);
            }
        }

        v[a.1].pop_front();
        if v[a.1].front().is_some() {
            if map.contains_key(&v[a.1].front().unwrap()) {
                q.push_back((map.get(&v[a.1].front().unwrap()).unwrap().clone(), a.1));
            } else {
                map.insert(*v[a.1].front().unwrap(), a.1);
            }
        }
    }

    let mut is_none = true;
    for i in 0..m {
        if !v[i].is_empty() {
            is_none = false;
            break;
        }
    }

    println!("{}", if !is_none { "No" } else { "Yes" });
}
