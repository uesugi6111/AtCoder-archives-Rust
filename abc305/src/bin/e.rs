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
        k: usize,
        ab: [(usize, usize); m],
        ph: [(usize, i64); k]
    );
    let mut e = vec![vec![]; n];

    let mut g = vec![0; n];
    for (a, b) in ab {
        e[a - 1].push(b - 1);
        e[b - 1].push(a - 1);
    }

    for (p, h) in ph.iter() {
        g[*p - 1] = *h + 1;
    }

    let mut bmap = std::collections::BTreeMap::new();
    for (p, h) in &ph {
        bmap.entry(*h + 1).or_insert_with(Vec::new).push(*p - 1);
    }

    let max = ph.iter().map(|x| x.1).max().unwrap() + 1;

    for i in 0..=max {
        let m = max - i;
        let buff = bmap.get(&m).unwrap_or(&vec![]).clone();

        for j in buff {
            for &l in &e[j] {
                if g[l] < m - 1 {
                    g[l] = m - 1;
                    bmap.entry(m - 1).or_insert_with(|| Vec::new()).push(l);
                }
            }
        }
    }

    let gg = g
        .iter()
        .enumerate()
        .filter(|x| *x.1 > 0)
        .map(|x| x.0)
        .collect::<Vec<usize>>();
    println!("{}", gg.len());
    for i in gg {
        print!("{} ", i + 1);
    }
}
