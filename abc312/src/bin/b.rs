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
    input!(n: usize, m: usize, s: [Chars; n]);
    let mut ans = vec![];

    for i in 0..n - 8 {
        'aaa: for j in 0..m - 8 {
            for k in 0..3 {
                for l in 0..3 {
                    if s[i + k][j + l] == '.' {
                        continue 'aaa;
                    }
                }
            }
            for k in 0..3 {
                for l in 0..3 {
                    if s[i + 6 + k][j + 6 + l] == '.' {
                        continue 'aaa;
                    }
                }
            }

            for k in 0..4 {
                if s[i + 3][j + k] == '#' {
                    continue 'aaa;
                }
            }

            for k in 0..4 {
                if s[i + k][j + 3] == '#' {
                    continue 'aaa;
                }
            }

            for k in 0..4 {
                if s[i + 5][j + 5 + k] == '#' {
                    continue 'aaa;
                }
            }

            for k in 0..4 {
                if s[i + 5 + k][j + 5] == '#' {
                    continue 'aaa;
                }
            }

            ans.push((i, j));
        }
    }

    for (i, j) in ans {
        println!("{} {}", i + 1, j + 1);
    }
}
