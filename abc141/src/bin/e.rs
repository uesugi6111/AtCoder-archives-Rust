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
#[proconio::fastout]
fn main() {
    input!(n: usize, s: Chars);
    let mut ans = 0;
    for i in 0..n {
        let z = z_algorithm(&s[i..n]);
        let max = z
            .iter()
            .enumerate()
            .map(|(i, &v)| i.min(v))
            .max()
            .unwrap_or(0);
        ans = ans.max(max);
    }
    println!("{}", ans);
}
pub fn z_algorithm(s: &[char]) -> Vec<usize> {
    let length = s.len();
    let mut z_array = vec![0_usize; length];

    z_array[0] = length;
    let (mut i, mut j) = (1, 0);

    while i < length {
        while i + j < length && s[j] == s[i + j] {
            j += 1;
        }

        z_array[i] = j;

        if j == 0 {
            i += 1;
            continue;
        }
        let mut k = 1;
        while k < j && k + z_array[k] < j {
            z_array[i + k] = z_array[k];
            k += 1;
        }
        i += k;
        j -= k;
    }
    z_array
}
