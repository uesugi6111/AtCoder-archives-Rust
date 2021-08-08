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
    input!(h: usize, w: usize, n: usize, ab: [(i64, i64); n]);

    let mut a_set = std::collections::HashSet::new();
    let mut b_set = std::collections::HashSet::new();

    for i in 0..n {
        a_set.insert(ab[i].0);
        b_set.insert(ab[i].1);
    }
    let mut a_v = a_set.iter().cloned().collect::<Vec<_>>();
    let mut b_v = b_set.iter().cloned().collect::<Vec<_>>();
    a_v.sort();
    b_v.sort();

    for i in 0..n {
        println!(
            "{} {}",
            binary_search(&a_v, ab[i].0 + 1),
            binary_search(&b_v, ab[i].1 + 1)
        )
    }
}

fn binary_search(v: &[i64], n: i64) -> i64 {
    let mut left = -1;
    let mut right = v.len() as i64;

    while right - left > 1 {
        let mid = left + (right - left) / 2;

        if is_ok(v, mid, n) {
            left = mid;
        } else {
            right = mid;
        };
    }

    right
}

fn is_ok(v: &[i64], mid: i64, n: i64) -> bool {
    v[mid as usize] < n
}
