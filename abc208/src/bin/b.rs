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
    input!(n: i64);

    let mut v = vec![0; 9];
    for i in 0..9 {
        v[i] = factorial(i as i64 + 2);
    }
    let mut count = 0;
    let mut nn = n;
    for &i in v.iter().rev() {
        while nn >= i {
            nn -= i;
            count += 1;
        }
    }
    println!("{}", count + nn);
}

fn factorial(num: i64) -> i64 {
    let current = num;
    if num <= 1 {
        return 1;
    }
    current * factorial(num - 1)
}

#[test]
fn a() {
    let mut buff: u128 = 1;
    let mut count = 0;
    while count < 100 {
        let mut is_ok = true;
        let mut b = 0;
        for i in (buff * 7).to_string().chars() {
            b += i as i64 - 48;
            if b > 2 {
                is_ok = false;
                break;
            }
        }
        if is_ok {
            count += 1;
            println!("{} * 7 = {}", buff, buff * 7);
        }
        buff += 1;
    }
}
