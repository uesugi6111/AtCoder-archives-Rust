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
    input!(w: i64,h:i64,n:usize,pq:[(i64,i64);n],aa:usize,mut a:[i64;aa],bb:usize,mut b:[i64;bb]);
    a.push(0);
    a.push(w);
    b.push(0);
    b.push(h);
    let aaa = a
        .iter()
        .copied()
        .collect::<std::collections::BTreeSet<i64>>();
    let bbb = b
        .iter()
        .copied()
        .collect::<std::collections::BTreeSet<i64>>();

    let mut map = std::collections::HashMap::new();
    for (p, q) in pq {
        let pp = aaa.range(p..).next().unwrap();
        let qq = bbb.range(q..).next().unwrap();
        *map.entry(pp)
            .or_insert_with(std::collections::HashMap::new)
            .entry(qq)
            .or_default() += 1;
    }

    let mut min = n;
    let mut max = 0;
    let mut count = 0;
    for (k, v) in map {
        for (kk, vv) in v {
            count += 1;
            min = min.min(vv);
            max = max.max(vv);
        }
    }
    if count < (a.len() - 1) * (b.len() - 1) {
        min = 0;
    }
    println!("{} {}", min, max);
}
