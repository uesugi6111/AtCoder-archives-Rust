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

    let mut set = std::collections::HashSet::new();

    let mut q = std::collections::VecDeque::new();

    q.push_back((1, 1));

    while let Some((y, x)) = q.pop_front() {
        if set.contains(&(y, x)) {
            continue;
        }
        set.insert((y, x));
        for i in 1..n {
            if s[y - i][x] == '#' {
                for j in 0..i - 1 {
                    set.insert((y - j, x));
                }
                if !set.contains(&(y - i, x)) && s[y - i + 1][x] == '.' {
                    q.push_back((y - i + 1, x));
                }
                break;
            }
        }
        for i in 1..m {
            if s[y][x - i] == '#' {
                for j in 0..i - 1 {
                    set.insert((y, x - j));
                }
                if !set.contains(&(y, x - i)) && s[y][x - i + 1] == '.' {
                    q.push_back((y, x - i + 1));
                }
                break;
            }
        }
        for i in 1..n {
            if s[y + i][x] == '#' {
                for j in 0..i - 1 {
                    set.insert((y + j, x));
                }
                if !set.contains(&(y + i - 1, x)) && s[y + i - 1][x] == '.' {
                    q.push_back((y + i - 1, x));
                }
                break;
            }
        }
        for i in 1..m {
            if s[y][x + i] == '#' {
                for j in 0..i - 1 {
                    set.insert((y, x + j));
                }
                if !set.contains(&(y, x + i - 1)) && s[y][x + i - 1] == '.' {
                    q.push_back((y, x + i - 1));
                }
                break;
            }
        }
    }

    println!("{}", set.len());
}
