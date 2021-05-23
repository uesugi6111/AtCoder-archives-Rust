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
    input!(n: usize, a: [i64; n], b: [i64; n]);
    let mut map = std::collections::HashMap::new();

    for i in 0..n {
        map.entry(i + b[i] as usize)
            .or_insert_with(|| std::collections::VecDeque::new())
            .push_back(i);
    }

    let mut c = vec![0i64; n];
    for i in 0..n {
        let v = map
            .get_mut(&(i + a[i] as usize))
            .unwrap_or(&mut std::collections::VecDeque::new())
            .pop_front();
        match v {
            Some(vv) => c[i] = vv as i64,
            None => {
                println!("-1");
                return;
            }
        }
    }

    println!("{}", a::inversion_number(&c));
}

mod a {
    pub fn inversion_number(array: &[i64]) -> i64 {
        count_merge(
            &mut array.iter().copied().collect::<Vec<_>>(),
            0..array.len(),
        )
    }
    fn count_merge(array: &mut Vec<i64>, range: std::ops::Range<usize>) -> i64 {
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
}
