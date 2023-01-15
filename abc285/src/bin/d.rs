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
    input!(n: usize, st: [(String, String); n]);

    let mut map = std::collections::HashMap::new();
    for (s, t) in st.iter() {
        map.insert(s.to_owned(), t.to_owned());
    }

    let mut memo = std::collections::HashMap::new();

    for (k, v) in st {
        f(&map, &mut memo, &k);
    }

    let mut ans = memo.iter().map(|(k, v)| v).all(|x| *x);

    println!("{}", if ans { "Yes" } else { "No" });
}

fn f(
    map: &std::collections::HashMap<String, String>,
    memo: &mut std::collections::HashMap<String, bool>,
    k: &str,
) -> bool {
    if let Some(&x) = memo.get(k) {
        if x == true {
            return true;
        } else {
            return false;
        }
    }

    memo.insert(k.to_owned(), false);

    let ans = if let Some(to) = map.get(k) {
        f(map, memo, to)
    } else {
        true
    };

    memo.insert(k.to_owned(), ans);
    ans
}
