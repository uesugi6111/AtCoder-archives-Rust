#[rustfmt::skip]
mod fast_input {
    #[macro_export] macro_rules! input{(sc=$sc:expr,$($r:tt)*)=>{input_inner!{$sc,$($r)*}};($($r:tt)*)=>{let mut sc=fast_input::Scanner::new(std::io::stdin().lock(),4096);input_inner!{sc,$($r)*}};}
    #[macro_export] macro_rules! input_inner{($sc:expr)=>{};($sc:expr,)=>{};($sc:expr,$var:ident:$t:tt$($r:tt)*)=>{let $var=read_value!($sc,$t);input_inner!{$sc $($r)*}};}
    #[macro_export] macro_rules! read_value{($sc:expr,($($t:tt),*))=>{($(read_value!($sc,$t)),*)};($sc:expr,[$t:tt;$len:expr])=>{(0..$len).map(|_|read_value!($sc,$t)).collect::<Vec<_>>()};($sc:expr,Chars)=>{read_value!($sc,String).chars().collect::<Vec<char>>()};($sc:expr,Usize1)=>{read_value!($sc,usize)-1};($sc:expr,$t:ty)=>{$sc.next::<$t>()};}
    pub struct Scanner {buf:Vec<u8>,pos: usize,}
    impl Scanner {
        pub fn new<R: std::io::Read>(mut reader: R, estimated: usize) -> Self {
            let mut buf = Vec::with_capacity(estimated);let _=std::io::copy(&mut reader,&mut buf).unwrap();if buf.last()!=Some(&b'\n'){panic!("{}", 0);}
            Scanner { buf, pos: 0 }
        }
        #[inline]
        pub fn next<T: std::str::FromStr>(&mut self) -> T where T::Err: std::fmt::Debug,{
            let mut start=None;loop{match(self.buf[self.pos],start.is_some()){(b' ',true)|(b'\n', true)=>break,(_, true)|(b' ', false)|(b'\n',false)=>self.pos+=1,(_, false)=>start=Some(self.pos),}}let target=&self.buf[start.unwrap()..self.pos];
            unsafe { std::str::from_utf8_unchecked(target) }.parse().unwrap()
        }
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
            if rh::rolling_hash(&s, i, mid) {
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
    pub fn rolling_hash(s: &[char], start: usize, l: usize) -> bool {
        let b: u128 = 1_000_000_007;
        let target_start = start + l;

        let pow_b = b.wrapping_pow(l as u32);

        let mut target_hash: u128 = 0;
        let mut base_hash: u128 = 0;
        for k in 0..l {
            base_hash = base_hash.wrapping_mul(b) + s[start + k] as u128;
            target_hash = target_hash.wrapping_mul(b) + s[target_start + k] as u128;
        }

        for k in 0..s.len() - target_start {
            if target_hash == base_hash {
                return true;
            }
            if target_start + l + k < s.len() {
                target_hash = target_hash
                    .wrapping_mul(b)
                    .wrapping_add(s[target_start + l + k] as u128)
                    .wrapping_sub((s[target_start + k] as u128).wrapping_mul(pow_b));
            }
        }
        false
    }
}
