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
    input!(n: usize, m: i64, l: [i64; n]);

    let bs = bs::BinarySearch::new(
        &l,
        *l.iter().max().unwrap(),
        l.iter().sum::<i64>() + n as i64,
    );

    let ans = bs.search(|&l, w| {
        let mut h_count = 1;
        let mut w_count = 0;
        for &i in l {
            let ii = i + 1;
            if w_count + ii > w {
                h_count += 1;
                w_count = ii;
            } else {
                w_count += ii;
            }
        }

        h_count <= m
    });

    println!("{}", ans);
}

mod bs {
    pub struct BinarySearch<T> {
        target: T,
        min: i64,
        max: i64,
    }
    impl<T> BinarySearch<T> {
        pub fn new(target: T, min: i64, max: i64) -> Self {
            Self { target, min, max }
        }
        /// f が true を帰す最小値を探す
        pub fn search<F>(&self, f: F) -> i64
        where
            F: Fn(&T, i64) -> bool,
        {
            let mut left = self.min;
            let mut right = self.max;

            while right - left > 1 {
                let mid = left + (right - left) / 2;

                if f(&self.target, mid) {
                    right = mid;
                } else {
                    left = mid;
                };
            }
            left
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        #[test]
        fn test_binary_search() {
            let v = (0_i64..100).filter(|x| x % 2 == 0).collect::<Vec<_>>();
            let bs = BinarySearch::new(&v, -1, v.len() as i64);
            for &i in v.iter() {
                let ans = bs.search(|x, j| x[j as usize] < i);
                assert_eq!(v[ans as usize], i)
            }
        }
    }
}
