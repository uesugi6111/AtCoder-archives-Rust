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
    input!(sc = sc, n: usize, q: usize);

    let mut hako = std::collections::BTreeMap::new();
    let mut card = std::collections::BTreeMap::new();

    for _ in 0..q {
        input!(sc = sc, p: usize);

        if p == 1 {
            input!(sc = sc, i: i64, j: i64);
            *hako
                .entry(j)
                .or_insert(std::collections::BTreeMap::new())
                .entry(i)
                .or_insert(0) += 1;
            card.entry(i)
                .or_insert(std::collections::BTreeSet::new())
                .insert(j);
        } else if p == 2 {
            input!(sc = sc, i: i64);
            for (k, v) in hako.get(&i).unwrap() {
                for i in 0..*v {
                    print!("{} ", *k);
                }
            }
            println!();
        } else {
            input!(sc = sc, i: i64);
            for v in card.get(&i).unwrap() {
                print!("{} ", *v);
            }
            println!();
        }
    }
}
