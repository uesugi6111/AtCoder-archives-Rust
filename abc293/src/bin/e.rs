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
    input!(a: i64, x: i64, m: i64);
    if a == 1 {
        println!("{}", a * x % m);
        return;
    }

    println!("{}", (modpow(a, x, m * (a - 1)) - 1) / (a - 1) % m);
}

fn sum(a: i64, r: i64, n: i64, m: i64) -> i64 {
    if n == 1 {
        return a % m;
    }
    let x = sum(a, r, n / 2, m);
    let mut ret = (x + modpow(r, n / 2, m) * x) % m;
    if n & 1 != 0 {
        ret = (a + r * ret) % m;
    }
    ret
}

pub fn modpow(base: i64, exp: i64, n: i64) -> i64 {
    let (mut base, mut exp, n) = (base as u128, exp, n as u128);

    assert!(
        exp >= 0,
        "negative exponent cannot be used in modular exponentiation"
    );

    if exp == 0 {
        return 1;
    }

    let mut res = 1;
    base %= n;

    loop {
        if exp % 2 == 1 {
            res *= &base;
            res %= &n;
        }

        if exp == 1 {
            return res as i64;
        }

        exp /= 2;
        base *= base;
        base %= n;
    }
}

#[cfg(test)]
mod tests {
    use super::modpow;

    #[test]
    fn test_modpow() {
        assert_eq!(modpow(3, 5, 5), 3);
        assert_eq!(modpow(2, 32, 9), 4);
    }
}
