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
    input!(
        n: usize,
        q: usize,
       mut  s: Chars,
        query: [(i64, usize, usize); q]
    );

    let mut seg_min = lazy_segment_tree::LazySegmentTree::new(
        n + 1,
        || 1_i64 << 60,
        |s, t| std::cmp::min(*s, *t),
        |f, x| *f + *x,
        |f, g| *f + *g,
        || 0,
    );
    seg_min.set(0, 0);
    let mut a = 0;
    for i in 0..n {
        if s[i] == '(' {
            a += 1;
        } else {
            a -= 1;
        }
        seg_min.set(i + 1, a);
    }

    for (o, l, r) in query {
        if o == 1 {
            if s[l - 1] == '(' && s[r - 1] == ')' {
                seg_min.apply_range(l..r, -2);
            } else if s[l - 1] == ')' && s[r - 1] == '(' {
                seg_min.apply_range(l..r, 2);
            }
            s.swap(l - 1, r - 1);
        } else {
            let buff = seg_min.get(l - 1);
            println!(
                "{}",
                if buff == seg_min.get(r) && seg_min.prod(l..r) >= buff {
                    "Yes"
                } else {
                    "No"
                }
            );
        }
    }
}

pub mod lazy_segment_tree {
    type Range = std::ops::Range<usize>;

    pub struct LazySegmentTree<S, Op, E, F, Mapping, Composition, Id> {
        n: usize,
        size: usize,
        log: usize,
        data: Vec<S>,
        lazy: Vec<F>,
        op: Op,
        e: E,
        mapping: Mapping,
        composition: Composition,
        id: Id,
    }

    impl<S, Op, E, F, Mapping, Composition, Id> LazySegmentTree<S, Op, E, F, Mapping, Composition, Id>
    where
        S: Clone,
        E: Fn() -> S,
        F: Clone,
        Op: Fn(&S, &S) -> S,
        Mapping: Fn(&F, &S) -> S,
        Composition: Fn(&F, &F) -> F,
        Id: Fn() -> F,
    {
        pub fn new(
            n: usize,
            e: E,
            op: Op,
            mapping: Mapping,
            composition: Composition,
            id: Id,
        ) -> Self {
            let size = n.next_power_of_two() as usize;
            LazySegmentTree {
                n,
                size,
                log: size.trailing_zeros() as usize,
                data: vec![e(); 2 * size],
                lazy: vec![id(); size],
                e,
                op,
                mapping,
                composition,
                id,
            }
        }
        pub fn set(&mut self, mut index: usize, value: S) {
            assert!(index < self.n);
            index += self.size;
            for i in (1..=self.log).rev() {
                self.push(index >> i);
            }
            self.data[index] = value;
            for i in 1..=self.log {
                self.update(index >> i);
            }
        }

        pub fn get(&mut self, mut index: usize) -> S {
            assert!(index < self.n);
            index += self.size;
            for i in (1..=self.log).rev() {
                self.push(index >> i);
            }
            self.data[index].clone()
        }

        pub fn prod(&mut self, range: Range) -> S {
            let mut l = range.start;
            let mut r = range.end;
            assert!(l < r && r <= self.n);

            l += self.size;
            r += self.size;

            for i in (1..=self.log).rev() {
                if ((l >> i) << i) != l {
                    self.push(l >> i);
                }
                if ((r >> i) << i) != r {
                    self.push(r >> i);
                }
            }

            let mut sum_l = (self.e)();
            let mut sum_r = (self.e)();
            while l < r {
                if l & 1 != 0 {
                    sum_l = (self.op)(&sum_l, &self.data[l]);
                    l += 1;
                }
                if r & 1 != 0 {
                    r -= 1;
                    sum_r = (self.op)(&self.data[r], &sum_r);
                }
                l >>= 1;
                r >>= 1;
            }

            (self.op)(&sum_l, &sum_r)
        }

        pub fn all_prod(&self) -> S {
            self.data[1].clone()
        }

        pub fn apply(&mut self, mut index: usize, f: F) {
            assert!(index < self.n);
            index += self.size;
            for i in (1..=self.log).rev() {
                self.push(index >> i);
            }
            self.data[index] = (self.mapping)(&f, &self.data[index]);
            for i in 1..=self.log {
                self.update(index >> i);
            }
        }
        pub fn apply_range(&mut self, range: Range, f: F) {
            let mut l = range.start;
            let mut r = range.end;
            assert!(l <= r && r <= self.n);
            if l == r {
                return;
            }

            l += self.size;
            r += self.size;

            for i in (1..=self.log).rev() {
                if ((l >> i) << i) != l {
                    self.push(l >> i);
                }
                if ((r >> i) << i) != r {
                    self.push((r - 1) >> i);
                }
            }

            {
                let mut l = l;
                let mut r = r;
                while l < r {
                    if l & 1 != 0 {
                        self.all_apply(l, f.clone());
                        l += 1;
                    }
                    if r & 1 != 0 {
                        r -= 1;
                        self.all_apply(r, f.clone());
                    }
                    l >>= 1;
                    r >>= 1;
                }
            }

            for i in 1..=self.log {
                if ((l >> i) << i) != l {
                    self.update(l >> i);
                }
                if ((r >> i) << i) != r {
                    self.update((r - 1) >> i);
                }
            }
        }

        fn update(&mut self, k: usize) {
            self.data[k] = (self.op)(&self.data[2 * k], &self.data[2 * k + 1]);
        }
        fn all_apply(&mut self, k: usize, f: F) {
            self.data[k] = (self.mapping)(&f, &self.data[k]);
            if k < self.size {
                self.lazy[k] = (self.composition)(&f, &self.lazy[k]);
            }
        }
        fn push(&mut self, k: usize) {
            self.all_apply(2 * k, self.lazy[k].clone());
            self.all_apply(2 * k + 1, self.lazy[k].clone());
            self.lazy[k] = (self.id)();
        }
    }
}
