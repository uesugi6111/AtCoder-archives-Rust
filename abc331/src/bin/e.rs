use std::collections::HashSet;

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
    input!(
        n: usize,
        m: usize,
        l: usize,
        a: [i64; n],
        b: [i64; m],
        cd: [(usize, usize); l]
    );
    let mut map = std::collections::HashMap::new();
    for (c, d) in cd {
        map.entry(c - 1)
            .or_insert(std::collections::HashSet::new())
            .insert(d - 1);
    }
    let mut aa = a.iter().enumerate().collect::<Vec<_>>();
    aa.sort_by_key(|x| -x.1);
    let mut bb = b.iter().enumerate().collect::<Vec<_>>();
    bb.sort_by_key(|x| -x.1);

    let mut max = 0;

    for i in 0..n {
        let set = map.get(&aa[i].0);
        for j in 0..m {
            if set.is_some() && set.unwrap().contains(&bb[j].0) {
                continue;
            }

            max = max.max(aa[i].1 + bb[j].1);
            break;
        }
    }
    println!("{}", max);
}
