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
    input!(n: usize, s: Chars);
    for i in 0..n {
        let a = z_a::z_algorithm(&s, &s[i..s.len()]);

        println!("{}", a);
    }
}

mod z_a {
    //! Z algorithm
    pub fn z_algorithm(s: &[char], t: &[char]) -> i64 {
        let length = s.len();
        let mut z_array = vec![0_usize; length];

        let (mut i, mut j) = (0, 0);
        let mut sum = 0;
        while i < length {
            while i + j < length && j < t.len() && t[j] == s[i + j] {
                j += 1;
            }

            z_array[i] = j;
            sum += j as i64;
            if j == 0 {
                i += 1;
                continue;
            }
            let mut k = 1;
            while k < j && k + z_array[k] < j {
                z_array[i + k] = z_array[k];
                sum += z_array[k] as i64;
                k += 1;
            }
            i += k;
            j -= k;
        }
        sum
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        #[test]
        fn test_z_algorithm() {
            let case = vec![
                ("abcbcba", vec![7, 0, 0, 0, 0, 0, 1]),
                ("mississippi", vec![11, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
                ("ababacaca", vec![9, 0, 3, 0, 1, 0, 1, 0, 1]),
                ("aaaaa", vec![5, 4, 3, 2, 1]),
            ];

            for (s, ans) in case {
                let z = z_algorithm(&s.to_string().chars().collect::<Vec<_>>());
                assert_eq!(z, ans);
            }
        }
    }
}
