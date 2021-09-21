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
use std::io::Write;
fn main() {
    input!(n: usize, q: usize, a: [i64; n], query: [(usize, usize); q]);

    let dst = dst::DisjointSparseTable::<dst::Add>::new(&a);

    let out = std::io::stdout();
    let mut out = std::io::BufWriter::new(out.lock());

    for (l, r) in query {
        writeln!(&mut out, "{}", dst.fold(l..r)).ok();
    }
}

mod dst {
    use std::ops::Range;

    pub trait SemiGroup {
        type T: Clone;
        fn operate(a: &Self::T, b: &Self::T) -> Self::T;
    }

    pub struct Add {}
    impl SemiGroup for Add {
        type T = i64;

        fn operate(a: &Self::T, b: &Self::T) -> Self::T {
            *a + *b
        }
    }

    #[derive(Debug)]
    pub struct DisjointSparseTable<S: SemiGroup> {
        pub table: Vec<Vec<S::T>>,
    }

    impl<S: SemiGroup> DisjointSparseTable<S> {
        pub fn new(v: &[S::T]) -> Self {
            let size = (32 - (v.len() as u32).saturating_sub(1).leading_zeros()) as usize;
            let mut table = vec![];
            table.push(v.to_vec());

            (1..size).for_each(|i| {
                let mut tmp = v.to_vec();

                let span = 2i64.pow(i as u32) as usize;

                (0..(v.len() + (span * 2) - 1) / (span * 2)).for_each(|j| {
                    let start = span * 2 * j + span;

                    (0..span - 1)
                        .map(|k| start - 2 - k)
                        .filter(|&k| k + 1 < v.len())
                        .for_each(|k| {
                            tmp[k] = S::operate(&tmp[k], &tmp[k + 1]);
                        });

                    (0..span - 1)
                        .map(|k| k + start + 1)
                        .filter(|&k| k < v.len())
                        .for_each(|k| {
                            tmp[k] = S::operate(&tmp[k], &tmp[k - 1]);
                        });
                });
                table.push(tmp);
            });

            DisjointSparseTable { table }
        }

        pub fn fold(&self, range: Range<usize>) -> S::T {
            if range.len() == 1 {
                return self.table[0][range.start].clone();
            }
            let h = (32 - ((range.start ^ (range.end - 1)) as u32).leading_zeros()) as usize - 1;
            S::operate(&self.table[h][range.start], &self.table[h][range.end - 1])
        }
    }
}
