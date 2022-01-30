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
    let mut sc = io_pro::Scanner::new(std::io::stdin().lock());
    input!(sc = sc, n: usize);

    let mut a = vec![];
    for i in 1..2 * n {
        input!(sc = sc, aa: [u32; 2 * n - i]);
        a.push(aa);
    }

    let per = make_permutation(2 * n as u32);

    let mut max = 0;
    for v in per {
        let mut buff = 0;
        for j in 0..n {
            if v[2 * j] > v[2 * j + 1] {
                buff = 0;
                break;
            }
            let value = (v[2 * j] as usize, v[2 * j + 1] as usize);

            buff ^= a[value.0][value.1 - (value.0 + 1)];
        }

        max = max.max(buff);
    }

    println!("{}", max);
}

fn f(n: usize, set: &mut std::collections::HashSet<usize>, number: u32, v: &[Vec<u32>]) -> u32 {
    let mut max = 0;
    for i in 0..n {
        if set.contains(&i) {
            continue;
        }
        for j in 0..i {
            if set.contains(&j) {
                continue;
            }
            f(n, set)
        }
    }
}
