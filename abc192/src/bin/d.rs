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
            let mut start=None;loop{match(self.buf[self.pos],start.is_some()){(b' ',true)|(b'\n', true)|(b'\r', true)=>break,(_, true)|(b' ', false)|(b'\n',false)|(b'\r', false)=>self.pos+=1,(_, false)=>start=Some(self.pos),}}let target=&self.buf[start.unwrap()..self.pos];
            unsafe { std::str::from_utf8_unchecked(target) }.parse().unwrap()
        }
    }
}
#[proconio::fastout]
fn main() {
    input!(x: Chars, m: u64);
    let v: Vec<_> = x.iter().map(|x| *x as u64 - 48).collect();
    let d = *v.iter().max().unwrap();
    if x.len() == 1 {
        if (x[0] as u64) <= m {
            println!("1");
        } else {
            println!("0");
        }
        return;
    }

    let (mut ng, mut ok) = (1_000_000_007, 0);

    while ng - ok > 1 {
        let mid = (ng + ok) / 2;
        if execute(d + 1 + mid, &v, m) {
            ok = mid;
        } else {
            ng = mid;
        }
    }

    println!("{}", ok + 1);
}

fn execute(i: u64, v: &[u64], m: u64) -> bool {
    let mut buff = 0;
    for (j, k) in v.iter().rev().enumerate() {
        let (b, o) = i.overflowing_pow(j as u32);

        let (b2, o2) = k.overflowing_mul(b);

        let (b3, o3) = (buff as u64).overflowing_add(b2);
        if o || o2 || o3 {
            return false;
        }
        buff = b3;
    }
    buff <= m
}
