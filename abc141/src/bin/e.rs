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

    let (mut ng, mut ok) = (n + 1, 0);
    let mut mid;

    while ng - ok > 1 {
        mid = (ng + ok) / 2;
        let mut mached = false;

        for i in 0..n {
            if i + mid * 2 > n {
                break;
            }
            if rh::rolling_hash(&s[i..i + mid], &s[i + mid..s.len()]) {
                mached = true;
                break;
            }
        }
        if mached {
            ok = mid;
        } else {
            ng = mid;
        }
    }
    println!("{}", ok);
}

mod rh {

    //const BASE: u128 = 2_305_843_009_213_693_951; // 2^61-1
    const BASE: u128 = 1000000007;
    pub fn rolling_hash(s: &[char], t: &[char]) -> bool {
        let l = s.len();

        let pow_b = BASE.wrapping_pow(l as u32);

        let mut target_hash: u128 = 0;
        let mut base_hash: u128 = 0;
        for k in 0..l {
            base_hash = base_hash.wrapping_mul(BASE) + unsafe { *s.get_unchecked(k) } as u128;
            target_hash = target_hash.wrapping_mul(BASE) + unsafe { *t.get_unchecked(k) } as u128;
        }
        if target_hash == base_hash {
            return true;
        }
        for k in 0..t.len() - l {
            target_hash = target_hash
                .wrapping_mul(BASE)
                .wrapping_add(unsafe { *t.get_unchecked(l + k) } as u128)
                .wrapping_sub((unsafe { *t.get_unchecked(k) } as u128).wrapping_mul(pow_b));
            if target_hash == base_hash {
                return true;
            }
        }
        false
    }
}
