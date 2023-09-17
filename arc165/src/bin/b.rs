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
    input!(n: usize, k: usize, p: [i64; n]);

    let st = st::SparseTable::<st::Min>::new(&p);

    let mut count = 0;
    while count < n - k {
        let min = st.fold(count..count + k);
        if p[count] == min {
            let min2 = st.fold(count + 1..count + k + 1);
            if p[count + k] != min2 {
                break;
            }
        }
        count += 1;
    }

    for i in 0..count {
        print!("{} ", p[i]);
    }

    let mut v = p.iter().skip(count).take(k).copied().collect::<Vec<_>>();
    v.sort();
    for i in 0..k {
        print!("{} ", v[i]);
    }
    for i in 0..n - count - k {
        print!("{} ", p[count + k + i]);
    }
    println!();
}

mod st {
    //! SparseTable
    //! 冪等半群列にたいして区間[l,r) の結果を戻す
    //! 構築 O(NlogN) クエリO(1)
    //! min, max, gcd, lcm 等

    use std::ops::Range;

    /// 冪等半群
    pub trait Band {
        type T: Clone;
        fn operate(a: &Self::T, b: &Self::T) -> Self::T;
    }

    /// 最小値
    pub struct Min {}
    impl Band for Min {
        type T = i64;

        fn operate(a: &Self::T, b: &Self::T) -> Self::T {
            *a.min(b)
        }
    }

    /// SparseTable
    pub struct SparseTable<B: Band> {
        table: Vec<Vec<B::T>>,
    }

    impl<B: Band> SparseTable<B> {
        /// O(NlogN)
        pub fn new(v: &[B::T]) -> Self {
            let mut table = vec![v.to_vec()];

            for i in 1..64 - v.len().leading_zeros() as usize {
                let mut tmp = vec![];
                for j in 0..=v.len() - (1 << i) {
                    tmp.push(B::operate(
                        &table[i - 1][j],
                        &table[i - 1][j + (1 << (i - 1))],
                    ));
                }
                table.push(tmp);
            }

            SparseTable { table }
        }

        /// [l,r)
        /// O(1)
        pub fn fold(&self, range: Range<usize>) -> B::T {
            let i = 64 - (range.end - range.start).leading_zeros() as usize - 1;
            B::operate(
                &self.table[i][range.start],
                &self.table[i][range.end - (1 << i)],
            )
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        #[test]
        fn test_sparse_table() {
            let a = SparseTable::<Min>::new(&[2, 10, 1, 100]);
            for (l, r, ans) in [
                (0, 1, 2),
                (0, 2, 2),
                (0, 3, 1),
                (0, 4, 1),
                (1, 2, 10),
                (1, 3, 1),
                (1, 4, 1),
                (2, 3, 1),
                (2, 4, 1),
                (3, 4, 100),
            ]
            .iter()
            {
                assert_eq!(a.fold(*l..*r), *ans);
            }
        }
    }
}
