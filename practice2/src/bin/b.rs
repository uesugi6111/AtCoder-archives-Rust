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
    use bit::Sum;
    input!(
        n: usize,
        q: usize,
        a: [i64; n],
        tuv: [(usize, usize, usize); q]
    );

    let mut bit = bit::FenwickTree::new(n + 1);
    a.iter().enumerate().for_each(|(i, v)| bit.add(i + 1, *v));
    for (t, u, v) in tuv {
        if t == 0 {
            bit.add(u + 1, v as i64);
        } else {
            println!("{}", bit.sum((u + 1, v)));
        }
    }
}

mod bit {
    use std::clone::Clone;
    use std::convert::From;
    use std::ops::{Add, AddAssign, Sub};

    ///binaryIndexTree
    #[derive(Clone, Debug)]
    pub struct FenwickTree<T> {
        array: Vec<T>,
    }

    impl<T> FenwickTree<T>
    where
        T: Add + Sub + Clone + Copy + From<u8> + AddAssign,
    {
        pub fn new(size: usize) -> FenwickTree<T> {
            let v: Vec<T> = vec![T::from(0u8); size + 1];
            Self { array: v }
        }
        pub fn add(&mut self, mut i: usize, x: T) {
            while i < self.array.len() {
                self.array[i] += x;
                i += i & i.wrapping_neg();
            }
        }
    }

    pub trait Sum<T, U> {
        fn sum(&self, i: T) -> U;
    }

    impl<T> Sum<usize, T> for FenwickTree<T>
    where
        T: Add + Sub + Clone + Copy + From<u8> + AddAssign,
    {
        fn sum(&self, mut i: usize) -> T {
            if i == 0 {
                return T::from(0u8);
            }
            let mut s = T::from(0u8);

            while i > 0 {
                s += self.array[i];
                i -= i & i.wrapping_neg();
            }
            s
        }
    }

    impl<T> Sum<(usize, usize), T> for FenwickTree<T>
    where
        T: Add + Sub + Clone + Copy + From<u8> + AddAssign,
        T: std::ops::Sub<Output = T>,
    {
        fn sum(&self, i: (usize, usize)) -> T {
            let sum_l = <FenwickTree<T> as Sum<usize, T>>::sum(self, i.0 - 1);
            let sum_r = <FenwickTree<T> as Sum<usize, T>>::sum(self, i.1);
            sum_r - sum_l
        }
    }
}
