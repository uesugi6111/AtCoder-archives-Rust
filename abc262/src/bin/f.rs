use itertools::Itertools;

#[rustfmt::skip]
mod io_pro {
    #[macro_export] macro_rules! input{(sc=$sc:expr,$($r:tt)*)=>{input_inner!{$sc,$($r)*}};($($r:tt)*)=>{let mut sc=io_pro::Scanner::new(std::io::stdin().lock());input_inner!{sc,$($r)*}};}
    #[macro_export] macro_rules! input_inner{($sc:expr)=>{};($sc:expr,)=>{};($sc:expr,$var:ident:$t:tt$($r:tt)*)=>{let $var=read_value!($sc,$t);input_inner!{$sc $($r)*}};($sc:expr,mut $var:ident:$t:tt$($r:tt)*)=>{let mut $var=read_value!($sc,$t);input_inner!{$sc $($r)*}};}
    #[macro_export] macro_rules! read_value{($sc:expr,($($t:tt),*))=>{($(read_value!($sc,$t)),*)};($sc:expr,[$t:tt;$len:expr])=>{(0..$len).map(|_|read_value!($sc,$t)).collect::<Vec<_>>()};($sc:expr,Chars)=>{read_value!($sc,String).chars().collect::<Vec<char>>()};($sc:expr,Usize1)=>{read_value!($sc,usize)-1};($sc:expr,$t:ty)=>{$sc.next::<$t>()};}
    pub struct Scanner{s:Box<str>,input:std::iter::Peekable<std::str::SplitAsciiWhitespace<'static>>,}
    impl Scanner{
        pub fn new<R:std::io::Read>(mut reader:R)->Self{let mut sc=Scanner{s:{let mut s=String::new();reader.read_to_string(&mut s).unwrap();s.into_boxed_str()},input:"".split_ascii_whitespace().peekable(),};let s:&'static str=unsafe{std::mem::transmute(&*sc.s)};sc.input=s.split_ascii_whitespace().peekable();sc}
        #[inline]pub fn next<T:std::str::FromStr>(&mut self)->T where T::Err:std::fmt::Debug,{self.input.next().unwrap().parse::<T>().expect("Parse error")}
    }
}
#[proconio::fastout]
fn main() {
    input!(n: usize, k: usize, p: [i64; n]);
    if k == 0 {
        for i in 0..n {
            print!("{} ", p[i]);
        }
        return;
    }
    let st = st::SparseTable::<st::Min>::new(&p);
    let f = st.fold(0..k);
    let b = st.fold(n - k..n);
    let (pp, kk) = if f < b {
        let index = p.iter().find_position(|&&x| x == f).unwrap().0;
        let mut pp = p.iter().copied().collect::<std::collections::VecDeque<_>>();

        for i in 0..index {
            pp.pop_front();
        }
        (pp, index)
    } else {
        let index = p.iter().find_position(|&&x| x == f).unwrap().0;
        let mut pp = p.iter().copied().collect::<std::collections::VecDeque<_>>();

        for i in 0..n - index {
            let bb = pp.pop_back().unwrap();
            pp.push_front(bb);
        }
        (pp, n - index)
    };

    let mut segtree = segtree::SegmentTree::<segtree::Min>::from(
        pp.iter()
            .enumerate()
            .map(|x| (*x.1, x.0))
            .collect::<Vec<_>>(),
    );

    let mut kkk = k - kk;
    for i in 0..k - kk {
        let min = segtree.query(1 + i, 1 + i + kkk);

        for j in 1..min.1 {
            segtree.set(j, (std::i64::MAX, 0));
        }
        kkk -= min.1 - 1;
    }

    for i in 0..pp.len() {
        if segtree.get(i).0 == std::i64::MAX {
            continue;
        }
        print!("{} ", segtree.get(i).0);
    }
}

mod segtree {
    //! セグメントツリー
    use std::cmp::max;
    use std::cmp::min;

    pub trait Monoid {
        type T: Clone;
        fn identity_element() -> Self::T;
        fn binary_operation(a: &Self::T, b: &Self::T) -> Self::T;
    }
    pub struct Min {}
    impl Monoid for Min {
        type T = (i64, usize);
        #[inline]
        fn identity_element() -> Self::T {
            (std::i64::MAX, 0)
        }
        #[inline]
        fn binary_operation(a: &Self::T, b: &Self::T) -> Self::T {
            if a.0 < b.0 {
                *a
            } else {
                *b
            }
        }
    }
    pub struct Max {}
    impl Monoid for Max {
        type T = (i64, usize);
        #[inline]
        fn identity_element() -> Self::T {
            (std::i64::MIN, 0)
        }
        #[inline]
        fn binary_operation(a: &Self::T, b: &Self::T) -> Self::T {
            if a.0 > b.0 {
                *a
            } else {
                *b
            }
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
            let data = {
                let mut data = vec![M::identity_element(); 2 * size];
                data[size..(size + n)].clone_from_slice(&v);
                data
            };
            {
                let mut sg = SegmentTree { n, size, log, data };
                (1..size).rev().for_each(|i| sg.update(i));
                sg
            }
        }
    }
    impl<M: Monoid> SegmentTree<M> {
        pub fn query(&self, mut l: usize, mut r: usize) -> M::T {
            let (mut sml, mut smr) = (M::identity_element(), M::identity_element());
            l += self.size;
            r += self.size;

            while l < r {
                if l & 1 != 0 {
                    sml = M::binary_operation(&sml, unsafe { self.data.get_unchecked(l) });
                    l += 1;
                }
                if r & 1 != 0 {
                    r -= 1;
                    smr = M::binary_operation(unsafe { self.data.get_unchecked(r) }, &smr);
                }
                l >>= 1;
                r >>= 1;
            }

            M::binary_operation(&sml, &smr)
        }
        fn update(&mut self, k: usize) {
            *unsafe { self.data.get_unchecked_mut(k) } =
                M::binary_operation(unsafe { self.data.get_unchecked(2 * k) }, unsafe {
                    self.data.get_unchecked(2 * k + 1)
                });
        }
        pub fn set(&mut self, mut p: usize, x: M::T) {
            p += self.size;
            self.data[p] = x;
            (1..=self.log).for_each(|i| self.update(p >> i));
        }
        pub fn get(&self, i: usize) -> M::T {
            self.data[i + self.size].clone()
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
            loop {
                // do
                while l % 2 == 0 {
                    l >>= 1;
                }
                if !f(&M::binary_operation(&sm, unsafe {
                    self.data.get_unchecked(l)
                })) {
                    while l < self.size {
                        l *= 2;
                        let res = M::binary_operation(&sm, unsafe { self.data.get_unchecked(l) });
                        if f(&res) {
                            sm = res;
                            l += 1;
                        }
                    }
                    return l - self.size;
                }
                sm = M::binary_operation(&sm, unsafe { self.data.get_unchecked(l) });
                l += 1;
                // while

                let l = l as isize;
                if (l & -l) == l {
                    break;
                }
            }
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
            loop {
                // do
                r -= 1;
                while r > 1 && r % 2 == 1 {
                    r >>= 1;
                }
                if !f(&M::binary_operation(
                    unsafe { self.data.get_unchecked(r) },
                    &sm,
                )) {
                    while r < self.size {
                        r = 2 * r + 1;
                        let res = M::binary_operation(unsafe { self.data.get_unchecked(r) }, &sm);
                        if f(&res) {
                            sm = res;
                            r -= 1;
                        }
                    }
                    return r + 1 - self.size;
                }
                sm = M::binary_operation(unsafe { self.data.get_unchecked(r) }, &sm);
                // while

                let r = r as isize;
                if (r & -r) == r {
                    break;
                }
            }

            0
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        #[test]
        fn test_practice2_sample() {
            let a = vec![1, 2, 3, 2, 1];
            let cxy = vec![(2, 1, 5), (3, 2, 3), (1, 3, 1), (2, 2, 4), (3, 1, 3)];

            let mut st = SegmentTree::<Max>::from(a);
            let mut ans = vec![];
            for (c, x, y) in cxy {
                if c == 1 {
                    st.set(x as usize - 1, y);
                } else if c == 2 {
                    ans.push(st.query(x as usize - 1, y as usize));
                } else {
                    ans.push(st.max_right(x as usize - 1, |&v| y > v) as i64 + 1);
                }
            }

            assert_eq!(vec![3, 3, 2, 6], ans);
        }
    }
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
