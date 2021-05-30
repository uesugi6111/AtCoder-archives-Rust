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
    input!(n: usize, m: usize, xy: [(i64, i64); m]);
    let mut set = std::collections::HashSet::new();
    for v in xy {
        set.insert(format!("{}_{}", v.0, v.1));
    }

    let mut mae = std::collections::HashSet::new();
    mae.insert(n);
    let mut ato = std::collections::HashSet::new();

    for i in 0..n * 2 {
        for j in mae {
            if !set.contains(&format!("{}_{}", i + 1, j)) {
                ato.insert(j);
            }
            if j > 0 && set.contains(&format!("{}_{}", i + 1, j - 1)) {
                ato.insert(j - 1);
            }
            if j < n * 2 && set.contains(&format!("{}_{}", i + 1, j + 1)) {
                ato.insert(j + 1);
            }
        }
        mae = ato;
        ato = std::collections::HashSet::new();
    }
    println!("{}", mae.iter().count());
}
