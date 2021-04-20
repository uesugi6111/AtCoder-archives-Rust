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
    input!(n: usize, q: usize, a: [i64; n], cxy: [(i64, i64, i64); q]);
    let mut st = segtree::SegmentTree::<segtree::Max>::from(a);
    for (c, x, y) in cxy {
        if c == 1 {
            st.set(x as usize - 1, y);
        } else if c == 2 {
            println!("{}", st.query(x as usize - 1, y as usize));
        } else {
            println!("{}", st.max_right(x as usize - 1, |&v| y > v) + 1);
        }
    }
}
mod segtree {
    use std::cmp::max;
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
    pub struct Max {}
    impl Monoid for Max {
        type T = i64;
        fn identity_element() -> Self::T {
            std::i64::MIN
        }
        fn binary_operation(a: &Self::T, b: &Self::T) -> Self::T {
            max(*a, *b)
        }
    }

    pub struct SegmentTree<M>
    where
        M: Monoid,
    {
        n: usize,
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
            let mut ret = SegmentTree { n, size, log, data };
            (1..size).rev().for_each(|i| ret.update(i));
            ret
        }
    }
    impl<M: Monoid> SegmentTree<M> {
        pub fn query(&self, mut l: usize, mut r: usize) -> M::T {
            let (mut sml, mut smr) = (M::identity_element(), M::identity_element());
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
            (1..=self.log).for_each(|i| self.update(p >> i));
        }
        pub fn get(&self, i: usize) -> M::T {
            self.data[i].clone()
        }
        pub fn max_right<F>(&self, mut l: usize, f: F) -> usize
        where
            F: Fn(&M::T) -> bool,
        {
            assert!(l <= self.n);
            assert!(f(&M::identity_element()));
            if l == self.n {
                return self.n;
            }
            l += self.size;
            let mut sm = M::identity_element();
            while {
                // do
                while l % 2 == 0 {
                    l >>= 1;
                }
                if !f(&M::binary_operation(&sm, &self.data[l])) {
                    while l < self.size {
                        l *= 2;
                        let res = M::binary_operation(&sm, &self.data[l]);
                        if f(&res) {
                            sm = res;
                            l += 1;
                        }
                    }
                    return l - self.size;
                }
                sm = M::binary_operation(&sm, &self.data[l]);
                l += 1;
                // while
                {
                    let l = l as isize;
                    (l & -l) != l
                }
            } {}
            self.n
        }

        pub fn min_left<F>(&self, mut r: usize, f: F) -> usize
        where
            F: Fn(&M::T) -> bool,
        {
            assert!(r <= self.n);
            assert!(f(&M::identity_element()));
            if r == 0 {
                return 0;
            }
            r += self.size;
            let mut sm = M::identity_element();
            while {
                // do
                r -= 1;
                while r > 1 && r % 2 == 1 {
                    r >>= 1;
                }
                if !f(&M::binary_operation(&self.data[r], &sm)) {
                    while r < self.size {
                        r = 2 * r + 1;
                        let res = M::binary_operation(&self.data[r], &sm);
                        if f(&res) {
                            sm = res;
                            r -= 1;
                        }
                    }
                    return r + 1 - self.size;
                }
                sm = M::binary_operation(&self.data[r], &sm);
                // while
                {
                    let r = r as isize;
                    (r & -r) != r
                }
            } {}
            0
        }
    }
}
