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
    input!(n: usize);

    let mut a = vec![vec![0; n]; n];

    let mut count = 0;
    let mut nn = 1;
    for i in 0..n {
        for j in 0..2.min(n - i) {
            for k in 0..n - i {
                if count % 4 == 0 {
                    a[i / 2][i / 2 + k] = nn;
                } else if count % 4 == 1 {
                    a[i / 2 + 1 + k][n - 1 - (i / 2)] = nn;
                } else if count % 4 == 2 {
                    a[n - 1 - (i / 2)][n - 2 - (i / 2) - k] = nn;
                } else if count % 4 == 3 {
                    a[n - 1 - (i / 2) - k][i / 2 - 1] = nn;
                }
                nn += 1;
            }

            count += 1;
            if i == 0 {
                break;
            }
        }
    }

    for i in 0..n {
        for j in 0..n {
            print!(
                "{} ",
                if a[i][j] == 0 {
                    'T'.to_string()
                } else {
                    a[i][j].to_string()
                }
            );
        }
        println!();
    }
}
