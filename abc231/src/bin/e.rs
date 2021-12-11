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
    input!(n: usize, x: i64, a: [i64; n]);

    let mut map = std::collections::BTreeMap::new();

    map.insert(0, 0);
    map.insert((x / a[n - 1]) * a[n - 1], x / a[n - 1]);
    map.insert(((x / a[n - 1]) + 1) * a[n - 1], (x / a[n - 1]) + 1);

    for i in 0..n - 1 {
        let (&f0, &f1) = map.range(..x).last().unwrap();
        let c = (x - f0) / a[n - 2 - i];
        map.entry(f0 + a[n - 2 - i] * c).or_insert_with(|| f1 + c);

        map.entry(f0 + a[n - 2 - i] * (c + 1))
            .or_insert_with(|| f1 + (c + 1));
    }
    dbg!(&map);
    let ans = map
        .iter()
        .filter(|(&k, _)| k >= x)
        .map(|(&k, &v)| {
            let mut buff = k - x;
            let mut count = 0;
            for i in a.iter().rev() {
                let bb = buff / i;
                count += bb;
                buff -= bb * i;
            }

            v + count
        })
        .min()
        .unwrap();

    println!("{}", ans);
}
