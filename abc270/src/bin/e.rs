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
    input!(n: usize, k: i64, a: [i64; n]);

    let mut bt = std::collections::BTreeMap::new();

    for i in 0..n {
        *bt.entry(a[i]).or_insert(0) += 1;
    }

    let mut aa = bt.iter().map(|(&k, &v)| k * v).collect::<Vec<_>>();
    aa.sort();
    let cumsum = cumsum::cumsum(&aa);

    let mut index = 0;
    while cumsum[index] < k {
        index += 1;
    }

    let diff = k - cumsum[index - 1];
    let c = if 2 <= index {
        bt.iter().nth(index - 2).unwrap()
    } else {
        (&0, &0)
    };
    let aaa = bt.iter().nth(index - 1).unwrap();

    let div = diff / (*aaa.1);
    let mut s = diff - (*aaa.1) * div;

    for i in 0..n {
        if a[i] == *aaa.0 {
            if s > 0 {
                print!("{} ", a[i] - div - 1);
            } else {
                print!("{} ", a[i] - div);
            }
            s -= 1;
        } else {
            print!("{} ", (a[i] - *c.0).max(0));
        }
    }
}

mod cumsum {
    //! 累積和
    pub fn cumsum(v: &[i64]) -> Vec<i64> {
        (0..1)
            .chain(v.iter().scan(0, |c, &x| {
                *c += x;
                Some(*c)
            }))
            .collect()
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        #[test]
        fn test_cumsum() {
            let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
            let cumsum = cumsum(&v);
            assert_eq!(&cumsum, &[0, 1, 3, 6, 10, 15, 21, 28, 36, 45, 55]);
        }
    }
}
