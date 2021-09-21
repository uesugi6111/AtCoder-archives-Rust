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
fn main() {
    input!(n: usize);
    println!("Yes");
}

mod era {
    //! エラトステネス

    use std::collections::BTreeMap;

    pub struct LinearSieve {
        n: usize,
        pub table: Vec<i64>,
        pub primes: Vec<usize>,
    }
    impl LinearSieve {
        pub fn new(n: usize) -> Self {
            let mut primes = vec![];
            let mut table = vec![0_i64; n + 1];
            for i in 2..n + 1 {
                if table[i] == 0 {
                    primes.push(i);
                    table[i] = i as i64;
                }
                for &p in &primes {
                    if p * i > n {
                        break;
                    }
                    table[p * i] = p as i64;
                }
            }
            LinearSieve { n, table, primes }
        }

        pub fn factorize(&self, n: i64) -> BTreeMap<i64, i64> {
            assert!(self.n as i64 >= n);

            let mut map = BTreeMap::new();
            let mut target = n;

            while target > 1 {
                let p = self.table[target as usize];
                let mut count = 0;

                while self.table[target as usize] == p {
                    target /= p;
                    count += 1;
                }
                map.insert(p, count);
            }
            map
        }

        // 約数列挙
        pub fn divisors(&self, n: i64) -> Vec<i64> {
            assert_ne!(n, 0);

            let mut ret = vec![1];
            let factor = self.factorize(n);

            for (k, exp) in factor {
                for i in 0..ret.len() {
                    let mut v = 1;
                    for _ in 0..exp {
                        v *= k;
                        ret.push(ret[i] * v);
                    }
                }
            }
            ret
        }

        pub fn is_prime(&self, n: i64) -> bool {
            assert!(self.n as i64 >= n);
            self.table[n as usize] == n
        }
    }
}
