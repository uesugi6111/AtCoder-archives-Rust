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
    input!(n: usize, m: usize, a: [Chars; 2 * n]);

    let mut s = (0..n as i64 * 2)
        .map(|x| (x, 0))
        .collect::<Vec<(i64, i64)>>();

    for i in 0..m {
        for j in 0..n {
            let buff = janken(&a, i, s[2 * j].0 as usize, s[2 * j + 1].0 as usize);
            if buff == 0 {
                s[2 * j].1 += 1;
            } else if buff == 2 {
                s[2 * j + 1].1 += 1;
            }
        }
        dbg!(&s);
        s.sort_by_key(|x| -x.1 * 1000 + x.0);
    }
    for i in 0..s.len() {
        println!("{}", s[i].0 + 1);
    }
}

fn janken(a: &[Vec<char>], m: usize, x: usize, y: usize) -> i64 {
    let (xx, yy) = (aaa(a[x][m]), aaa(a[y][m]));
    if (xx + 1) % 3 == yy {
        0
    } else if xx == yy {
        1
    } else {
        2
    }
}

fn aaa(c: char) -> i64 {
    match c {
        'G' => 0,
        'C' => 1,
        _ => 2,
    }
}
