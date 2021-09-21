#[rustfmt::skip]
mod fast_input {
    #[macro_export] macro_rules! input{(sc=$sc:expr,$($r:tt)*)=>{input_inner!{$sc,$($r)*}};($($r:tt)*)=>{let mut sc=fast_input::Scanner::new(std::io::stdin().lock());input_inner!{sc,$($r)*}};}
    #[macro_export] macro_rules! input_inner{($sc:expr)=>{};($sc:expr,)=>{};($sc:expr,$var:ident:$t:tt$($r:tt)*)=>{let $var=read_value!($sc,$t);input_inner!{$sc $($r)*}};}
    #[macro_export] macro_rules! read_value{($sc:expr,($($t:tt),*))=>{($(read_value!($sc,$t)),*)};($sc:expr,[$t:tt;$len:expr])=>{(0..$len).map(|_|read_value!($sc,$t)).collect::<Vec<_>>()};($sc:expr,Chars)=>{read_value!($sc,String).chars().collect::<Vec<char>>()};($sc:expr,Usize1)=>{read_value!($sc,usize)-1};($sc:expr,$t:ty)=>{$sc.next::<$t>()};}
    pub struct Scanner {s:Box<str>,input: std::iter::Peekable<std::str::SplitAsciiWhitespace<'static>>,}
    impl Scanner {
        pub fn new<R: std::io::Read>(mut reader: R) -> Self {
            let s={let mut s = String::new();reader.read_to_string(&mut s).unwrap();s.into_boxed_str()};
            let mut sc=Scanner {s,input:"".split_ascii_whitespace().peekable(),};
            let s:&'static str=unsafe{std::mem::transmute(&*sc.s)};
            sc.input = s.split_ascii_whitespace().peekable();
            sc
        }
        #[inline] pub fn next<T: std::str::FromStr>(&mut self) -> T where T::Err: std::fmt::Debug,{self.input.next().unwrap().parse::<T>().expect("Parse error")}
    }
}
fn main() {
    input!(n: usize, q: usize, cxy: [(i64, i64, i64); q]);
    let mut st = segtree::SegmentTree::<segtree::Min>::new(n);
    for (c, x, y) in cxy {
        if c == 0 {
            st.set(x as usize, y);
        } else {
            println!("{}", st.query(x as usize, y as usize + 1));
        }
    }
}
mod segtree {
    use std::cmp::min;

    pub trait Monoid {
        type T: Clone;
        fn identity_element() -> Self::T;
        fn binary_operation(a: &Self::T, b: &Self::T) -> Self::T;
    }
    pub struct Min {}
    impl Monoid for Min {
        type T = i64;
        fn identity_element() -> Self::T {
            std::i32::MAX as i64
        }
        fn binary_operation(a: &Self::T, b: &Self::T) -> Self::T {
            min(*a, *b)
        }
    }

    pub struct SegmentTree<M>
    where
        M: Monoid,
    {
        size: usize,
        log: usize,
        data: Vec<M::T>,
    }
    impl<M: Monoid> SegmentTree<M> {
        pub fn new(n: usize) -> SegmentTree<M> {
            vec![M::identity_element(); n].into()
        }
    }
    impl<M: Monoid> From<Vec<M::T>> for SegmentTree<M> {
        fn from(v: Vec<M::T>) -> Self {
            let n = v.len();
            let log = (32 - (n as u32).saturating_sub(1).leading_zeros()) as usize;
            let size = 1 << log;
            let mut data = vec![M::identity_element(); 2 * size];
            data[size..(size + n)].clone_from_slice(&v);
            let mut ret = SegmentTree { size, log, data };
            for i in (1..size).rev() {
                ret.update(i);
            }
            ret
        }
    }
    impl<M: Monoid> SegmentTree<M> {
        pub fn query(&self, mut l: usize, mut r: usize) -> M::T {
            let mut sml = M::identity_element();
            let mut smr = M::identity_element();
            l += self.size;
            r += self.size;

            while l < r {
                if l & 1 != 0 {
                    sml = M::binary_operation(&sml, &self.data[l]);
                    l += 1;
                }
                if r & 1 != 0 {
                    r -= 1;
                    smr = M::binary_operation(&self.data[r], &smr);
                }
                l >>= 1;
                r >>= 1;
            }

            M::binary_operation(&sml, &smr)
        }
        fn update(&mut self, k: usize) {
            self.data[k] = M::binary_operation(&self.data[2 * k], &self.data[2 * k + 1]);
        }
        pub fn set(&mut self, mut p: usize, x: M::T) {
            p += self.size;
            self.data[p] = x;
            for i in 1..=self.log {
                self.update(p >> i);
            }
        }
    }
}
