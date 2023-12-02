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
    input!(
        n: usize,
        q: usize,
        p: [Chars; n],
        abcd: [(usize, usize, usize, usize); q]
    );

    let pp = p
        .iter()
        .map(|x| {
            x.iter()
                .map(|y| if *y == 'B' { 1 } else { 0 })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let cs = cs::CumSum2D::new(&pp);
    let nn = n as i64;
    for (a, b, c, d) in abcd {
        let ac = (((c as i64 / nn) * nn) - (((a as i64 + nn) / nn) * nn)) / nn;

        let hidari = if ac >= 0 {
            let hidari_ue = cs.query(a % n, n - 1, b % n, n - 1);
            let hidari_shita = cs.query(0, c % n, b % n, n - 1);
            let yoko = cs.query(0, n - 1, b % n, n - 1);
            hidari_ue + yoko * ac + hidari_shita
        } else {
            cs.query(a % n, c % n, b % n, n - 1)
        };

        let naka = if ac >= 0 {
            let ue = cs.query(a % n, n - 1, 0, n - 1);
            let shita = cs.query(0, c % n, 0, n - 1);
            let yoko = cs.query(0, n - 1, 0, n - 1);
            ue + yoko * ac + shita
        } else {
            cs.query(a % n, c % n, 0, n - 1)
        };

        let migi = if ac >= 0 {
            let migi_ue = cs.query(a % n, n - 1, 0, d % n);
            let migi_shita = cs.query(0, c % n, 0, d % n);
            let yoko = cs.query(0, n - 1, 0, d % n);
            migi_ue + yoko * ac + migi_shita
        } else {
            cs.query(a % n, c % n, 0, d % n)
        };

        let bd = (((d as i64 / nn) * nn) - (((b as i64 + nn) / nn) * nn)) / nn;
        let zenbu = if bd >= 0 {
            hidari + naka * bd + migi
        } else {
            if ac >= 0 {
                let ue = cs.query(a % n, n - 1, b % n, d % n);
                let shita = cs.query(0, c % n, b % n, d % n);
                let yoko = cs.query(0, n - 1, b % n, d % n);
                ue + yoko * ac + shita
            } else {
                cs.query(a % n, c % n, b % n, d % n)
            }
        };

        println!("{}", zenbu);
    }
}

mod cs {
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
