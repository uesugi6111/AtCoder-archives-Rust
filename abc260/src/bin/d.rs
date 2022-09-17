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
    input!(n: usize, k: usize, p: [i64; n]);
    let mut ans = vec![None; n];
    let mut v: Vec<Vec<i64>> = vec![];
    let mut map = std::collections::BTreeMap::<i64, usize>::new();

    for i in 0..n {
        let buff = map.range(p[i]..).next().map(|x| (*x.0, *x.1));

        match buff {
            Some(entry) => {
                map.remove(&entry.0);

                v[entry.1].push(p[i]);

                if v[entry.1].len() == k {
                    for j in v[entry.1].iter() {
                        ans[*j as usize - 1] = Some(i as i64 + 1);
                    }
                    v[entry.1].clear();
                } else {
                    map.insert(p[i], entry.1);
                }
            }
            None => {
                if k == 1 {
                    ans[p[i] as usize - 1] = Some(i as i64 + 1);
                } else {
                    v.push(vec![p[i]]);

                    map.insert(p[i], v.len() - 1);
                }
            }
        }
    }

    for i in 0..n {
        println!("{}", ans[i].unwrap_or(-1));
    }
}