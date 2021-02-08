#[rustfmt::skip]
mod fast_input {
    #[macro_export]macro_rules! input{($($r:tt)*)=>{let s={use std::io::Read;let mut s=String::new();std::io::stdin().read_to_string(&mut s).unwrap();let input=Box::leak(s.into_boxed_str());input};let mut iter=s.split_ascii_whitespace();input_inner!{iter,$($r)*}};}
    #[macro_export]macro_rules! input_inner{($iter:expr)=>{};($iter:expr,)=>{};($iter:expr,$var:ident:$t:tt$($r:tt)*)=>{let $var=read_value!($iter,$t);input_inner!{$iter $($r)*}};}
    #[macro_export]macro_rules! read_value{($iter:expr,($($t:tt),*))=>{($(read_value!($iter,$t)),*)};($iter:expr,[$t:tt;$len:expr])=>{(0..$len).map(|_|read_value!($iter,$t)).collect::<Vec<_>>()};($iter:expr,Chars)=>{read_value!($iter,String).chars().collect::<Vec<char>>()};($iter:expr,Usize1)=>{read_value!($iter,usize)-1};($iter:expr,$t:ty)=>{$iter.next().unwrap().parse::<$t>().expect("Parse error")};}
}
fn main() {
    input!(n: i64, a: i64, b: i64, c: i64);

    println!(
        "{}",
        dbg!(tousa_wa(n, a)) + tousa_wa(n, b) + tousa_wa(n, c)
            - tousa_wa(n, lcm(a, b))
            - tousa_wa(n, lcm(b, c))
            - tousa_wa(n, lcm(c, a))
            + tousa_wa(n, lcm(lcm(a, b), c))
    );
}
fn gcd(m: i64, n: i64) -> i64 {
    if m == 0 {
        n.abs()
    } else {
        gcd(n % m, m)
    }
}
fn lcm(m: i64, n: i64) -> i64 {
    m * n / gcd(m, n)
}

fn tousa_wa(n: i64, x: i64) -> i64 {
    (2 * x + (n / x - 1) * x) * (n / x) / 2
}

#[test]
fn aaa() {
    dbg!(tousa_wa(10, 1));
}
