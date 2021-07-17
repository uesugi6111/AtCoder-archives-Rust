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
    input!(n: usize, q: usize, a: [u64; n], txy: [(i32, u64, u64); q]);

    let mut ft = ft::FenwickTree::<ft::Xor>::new(n);

    for (i, &v) in a.iter().enumerate() {
        ft.add(i, v);
    }

    for (t, x, y) in txy {
        if t == 1 {
            ft.add(x as usize - 1, y);
        } else {
            println!("{}", ft.sum(y as usize) ^ ft.sum(x as usize - 1));
        }
    }
}

mod ft {
    //! BIT

    pub trait Monoid {
        type T: Clone;
        fn identity_element() -> Self::T;
        fn binary_operation(a: &Self::T, b: &Self::T) -> Self::T;
    }

    pub struct Xor {}
    impl Monoid for Xor {
        type T = u64;
        #[inline]
        fn identity_element() -> Self::T {
            0 as u64
        }
        #[inline]
        fn binary_operation(a: &Self::T, b: &Self::T) -> Self::T {
            *a ^ *b
        }
    }

    ///binaryIndexTree
    #[derive(Clone, Debug)]
    pub struct FenwickTree<M>
    where
        M: Monoid,
    {
        array: Vec<M::T>,
    }

    impl<M> FenwickTree<M>
    where
        M: Monoid,
    {
        #[inline]
        pub fn new(size: usize) -> FenwickTree<M> {
            Self {
                array: vec![M::identity_element(); size + 1],
            }
        }
        #[inline]
        pub fn add(&mut self, index: usize, x: M::T) {
            let mut i = index + 1;
            while i < self.array.len() {
                self.array[i] = M::binary_operation(&self.array[i], &x);
                i += i & i.wrapping_neg();
            }
        }

        #[inline]
        pub fn sum(&self, end: usize) -> M::T {
            let mut s = M::identity_element();
            let mut i = end;
            while i > 0 {
                s = M::binary_operation(&s, &self.array[i]);
                i -= i & i.wrapping_neg();
            }
            s
        }
    }
}
