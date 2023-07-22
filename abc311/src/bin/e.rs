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
    input!(h: usize, w: usize, n: usize, ab: [(usize, usize); n]);

    let mut g = vec![vec![0; w]; h];

    for (a, b) in ab {
        g[a - 1][b - 1] = 1;
    }

    let gg = cs2d::CumSum2D::new(&g);

    let mut count = 0;
    for i in 0..h {
        for j in 0..w {
            let c = (h - i).min(w - j);
            let bs = bs::BinarySearch::new(&gg, -1, c as i64);
            let buff = bs.search(|x, cc| x.query(i, i + cc as usize, j, j + cc as usize) == 0);
            count += buff;
        }
    }

    println!("{}", count);
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
                    left = mid;
                } else {
                    right = mid
                };
            }
            right
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

mod cs2d {

    //! 二次元累積和
    #[derive(Clone, Debug)]
    pub struct CumSum2D {
        v: Vec<Vec<i64>>,
    }

    impl CumSum2D {
        pub fn new(source: &[Vec<i64>]) -> Self {
            let h = source.len();
            let w = source[0].len();
            let mut v = vec![vec![0i64; w + 1]; h + 1];

            for i in 0..h {
                for j in 0..w {
                    v[i + 1][j + 1] = source[i][j] + v[i][j + 1] + v[i + 1][j] - v[i][j];
                }
            }
            CumSum2D { v }
        }

        pub fn query(&self, top: usize, bottom: usize, left: usize, right: usize) -> i64 {
            self.v[bottom + 1][right + 1] - self.v[bottom + 1][left] - self.v[top][right + 1]
                + self.v[top][left]
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_cumsum_2d() {
            let a = CumSum2D::new(&[
                vec![1, 2, 3, 4],
                vec![1, 2, 3, 4],
                vec![1, 2, 3, 4],
                vec![1, 2, 3, 4],
            ]);
            assert_eq!(
                a.v,
                vec![
                    vec![0, 0, 0, 0, 0],
                    vec![0, 1, 3, 6, 10],
                    vec![0, 2, 6, 12, 20],
                    vec![0, 3, 9, 18, 30],
                    vec![0, 4, 12, 24, 40]
                ]
            );
            assert_eq!(a.query(0, 0, 0, 0), 1);
            assert_eq!(a.query(0, 1, 0, 1), 6);
            assert_eq!(a.query(1, 2, 2, 3), 14);
            assert_eq!(a.query(0, 0, 0, 3), 10);
            assert_eq!(a.query(0, 3, 0, 0), 4);
            assert_eq!(a.query(3, 3, 3, 3), 4);

            assert_eq!(a.query(0, 3, 0, 3), 40);
        }
    }
}
