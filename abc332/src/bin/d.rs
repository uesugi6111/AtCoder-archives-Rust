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
    input!(h: usize, w: usize, a: [[i64; w]; h], b: [[i64; w]; h]);
    let mut min = std::i64::MAX;
    for v1 in p::Permutation::new(&(0usize..h).collect::<Vec<_>>()) {
        for v2 in p::Permutation::new(&(0usize..w).collect::<Vec<_>>()) {
            let c = (0..h)
                .map(|x| (0..w).map(|y| a[v1[x]][v2[y]]).collect::<Vec<_>>())
                .collect::<Vec<_>>();

            if b == c {
                min = min.min(in_num::inversion_number(&v1) + in_num::inversion_number(&v2));
            }
        }
    }
    println!("{}", if min == std::i64::MAX { -1 } else { min });
}

mod in_num {
    //! 転倒数
    pub fn inversion_number<T: Copy + PartialOrd>(array: &[T]) -> i64 {
        count_merge(&mut array.to_vec(), 0..array.len())
    }
    fn count_merge<T: Copy + PartialOrd>(array: &mut Vec<T>, range: std::ops::Range<usize>) -> i64 {
        let length = range.len() as i64;
        if length <= 1 {
            return 0;
        }

        let mut count = 0;
        let mid = (range.start + range.end) / 2;
        count += count_merge(array, range.start..mid);
        count += count_merge(array, mid..range.end);

        let b = array
            .iter()
            .skip(range.start)
            .take(mid - range.start)
            .copied()
            .collect::<Vec<_>>();
        let c = array
            .iter()
            .skip(mid)
            .take(range.end - mid)
            .copied()
            .collect::<Vec<_>>();

        let (mut ai, mut bi, mut ci) = (0, 0, 0);

        while ai < length {
            if bi < b.len() && (ci == c.len() || b[bi] <= c[ci]) {
                array[range.start + ai as usize] = b[bi];
                ai += 1;
                bi += 1;
            } else {
                count += length / 2 - bi as i64;
                array[range.start + ai as usize] = c[ci];
                ai += 1;
                ci += 1;
            }
        }
        count
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        #[test]
        fn test_inversion_number() {
            let array = {
                let v = vec![
                    (vec![3, 1, 5, 4, 2], 5),
                    (vec![3, 5, 2, 1, 4], 6),
                    (vec![3, 1, 2], 2),
                    (vec![6, 1, 5, 8, 2, 3, 4, 7], 12),
                    (vec![7, 6, 1, 5, 8, 2, 3, 10, 4, 9], 19),
                    (
                        vec![
                            63, 16, 24, 7, 29, 57, 65, 26, 36, 32, 50, 5, 34, 1, 18, 15, 49, 9, 47,
                            53, 10, 35, 76, 79,
                        ],
                        122,
                    ),
                    (
                        vec![
                            0, 18, 35, 2, 31, 33, 32, 6, 11, 15, 36, 19, 42, 23, 9, 20, 24, 3, 10,
                            47, 8, 38, 5, 37, 46,
                        ],
                        125,
                    ),
                ];
                v
            };

            for (input, ans) in array {
                assert_eq!(inversion_number(&input), ans);
            }
        }
    }
}

mod p {
    pub struct Permutation<T>
    where
        T: Clone,
    {
        p: Vec<T>,
        init: bool,
    }

    impl<T> Permutation<T>
    where
        T: Clone,
    {
        pub fn new(p: &[T]) -> Self {
            Self {
                p: p.to_vec(),
                init: false,
            }
        }
    }
    impl<T> Iterator for Permutation<T>
    where
        T: Clone + Ord,
    {
        type Item = Vec<T>;

        fn next(&mut self) -> Option<Self::Item> {
            if !self.init {
                self.p.sort();
                self.init = true;
                return Some(self.p.clone());
            }
            let Some(i) = (0..&self.p.len() - 1).rfind(|&i| self.p[i] < self.p[i + 1]) else {
                return None;
            };
            let j = self.p.iter().rposition(|x| x > &self.p[i]).unwrap();
            self.p.swap(i, j);
            self.p[i + 1..].reverse();
            Some(self.p.clone())
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_struct() {
            let expect = [
                &[0, 1, 2],
                &[0, 2, 1],
                &[1, 0, 2],
                &[1, 2, 0],
                &[2, 0, 1],
                &[2, 1, 0],
            ];
            let a = Permutation::new(&[0, 1, 2]);

            for (i, v) in a.enumerate() {
                assert_eq!(v, expect[i]);
            }
        }
    }
}
