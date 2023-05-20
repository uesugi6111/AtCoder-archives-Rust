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
    let snuke = ['s', 'n', 'u', 'k', 'e'];
    input!(h: usize, w: usize, s: [Chars; h]);
    for i in 0..h {
        for j in 0..w {
            let mut is_ok = true;
            for k in 0..5 {
                if w <= j + 4 || snuke[k] != s[i][j + k] {
                    is_ok = false;
                    break;
                }
            }
            if is_ok {
                for l in 0..5 {
                    println!("{} {}", i + 1, j + 1 + l);
                }
                return;
            } else {
                is_ok = true;
            }

            for k in 0..5 {
                if w <= j + 4 || snuke[k] != s[i][j + 4 - k] {
                    is_ok = false;
                    break;
                }
            }
            if is_ok {
                for l in 0..5 {
                    println!("{} {}", i + 1, j + 1 + 4 - l);
                }
                return;
            } else {
                is_ok = true;
            }

            for k in 0..5 {
                if h <= i + 4 || snuke[k] != s[i + k][j] {
                    is_ok = false;
                    break;
                }
            }
            if is_ok {
                for l in 0..5 {
                    println!("{} {}", i + 1 + l, j + 1);
                }
                return;
            } else {
                is_ok = true;
            }
            for k in 0..5 {
                if h <= i + 4 || snuke[k] != s[i + 4 - k][j] {
                    is_ok = false;
                    break;
                }
            }
            if is_ok {
                for l in 0..5 {
                    println!("{} {}", i + 1 + 4 - l, j + 1);
                }
                return;
            } else {
                is_ok = true;
            }

            for k in 0..5 {
                if w <= j + 4 || h <= i + 4 || snuke[k] != s[i + k][j + k] {
                    is_ok = false;
                    break;
                }
            }
            if is_ok {
                for l in 0..5 {
                    println!("{} {}", i + 1 + l, j + 1 + l);
                }
                return;
            } else {
                is_ok = true;
            }

            for k in 0..5 {
                if w <= j + 4 || h <= i + 4 || snuke[k] != s[i + 4 - k][j + 4 - k] {
                    is_ok = false;
                    break;
                }
            }
            if is_ok {
                for l in 0..5 {
                    println!("{} {}", i + 1 + 4 - l, j + 1 + 4 - l);
                }
                return;
            } else {
                is_ok = true;
            }

            for k in 0..5 {
                if w <= j + 4 || i < 4 || snuke[k] != s[i - k][j + k] {
                    is_ok = false;
                    break;
                }
            }
            if is_ok {
                for l in 0..5 {
                    println!("{} {}", i + 1 - l, j + 1 + l);
                }
                return;
            } else {
                is_ok = true;
            }

            for k in 0..5 {
                if w <= j + 4 || i < 4 || snuke[k] != s[i - 4 + k][j + 4 - k] {
                    is_ok = false;
                    break;
                }
            }
            if is_ok {
                for l in 0..5 {
                    println!("{} {}", i + 1 - 4 + l, j + 1 + 4 - l);
                }
                return;
            }
        }
    }
}
