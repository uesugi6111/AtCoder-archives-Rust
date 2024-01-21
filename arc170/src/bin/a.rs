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
    input!(n: usize, s: Chars, t: Chars);
    let mut count_a = 0i64;

    let mut a_flag = false;

    for i in 0..n {
        if t[i] == 'B' && s[i] == 'A' {
            if !a_flag {
                println!("-1");
                return;
            }
        }
        if t[i] == 'A' {
            a_flag = true;
        }
    }
    let mut count_b = 0i64;
    let mut b_flag = false;
    for i in 0..n {
        if t[n - 1 - i] == 'A' && s[n - 1 - i] == 'B' {
            if !b_flag {
                println!("-1");
                return;
            }
        }
        if t[n - 1 - i] == 'B' {
            b_flag = true;
        }
    }

    for i in 0..n {
        if t[i] == 'B' && s[i] == 'A' {
            count_a += 1;
            if count_b > 0 {
                count_b -= 1;
            }
        }
        if t[i] == 'A' && s[i] == 'B' {
            count_b += 1;
        }
    }

    println!("{}", count_a + count_b);
    //println!("{}", tt(n, &s, &t))
}

fn tt(n: usize, s: &[char], t: &[char]) -> i64 {
    let mut s = s.to_vec();
    let mut t = t.to_vec();
    let mut aa: i64 = 0;

    let mut count_mae = 0;
    while count_mae < n {
        if t[count_mae] == 'A' && s[count_mae] == 'B' {
            let mut b = std::usize::MAX;
            for i in count_mae + 1..n {
                if t[i] == 'B' && s[i] == 'A' {
                    b = i;
                    break;
                }
            }
            if b == std::usize::MAX {
                for i in count_mae + 1..n {
                    if t[i] == 'B' {
                        b = i;
                        break;
                    }
                }
            }
            if b == std::usize::MAX {
                return -1;
            }
            s[count_mae] = 'A';
            s[b] = 'B';
            aa += 1
        }
        count_mae += 1;
    }
    let mut bb = 0;
    let mut count_usiro = 0;
    while count_usiro < n {
        if t[n - 1 - count_usiro] == 'A' && s[n - 1 - count_usiro] == 'B' {
            let mut b = std::usize::MAX;
            for i in count_usiro + 1..n {
                if t[n - 1 - i] == 'A' && s[n - 1 - i] == 'B' {
                    b = i;
                    break;
                }
            }
            if b == std::usize::MAX {
                for i in count_usiro + 1..n {
                    if t[n - 1 - i] == 'A' {
                        b = i;
                        break;
                    }
                }
            }
            if b == std::usize::MAX {
                return -1;
            }
            s[n - 1 - count_usiro] = 'A';
            s[n - 1 - b] = 'B';
            bb += 1
        }
        count_usiro += 1;
    }
    if s == t {
        aa + bb
    } else {
        -1
    }
}
