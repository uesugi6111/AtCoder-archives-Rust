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
    input!(t: usize, xypq: [(usize, usize, usize, usize); t]);

    for (x, y, p, q) in xypq {
        let xy = 2 * (x + y);
        let pq = p + q;
        let lcm = lcm(xy as i64, pq as i64);
        let mut tree_map = std::collections::BTreeMap::new();
        for i in 0..lcm {
            let xy_m = i % xy as i64;
            let pq_m = i % pq as i64;

            if xy_m == x as i64 {
                tree_map.insert(i, 1);
            }
            if xy_m == (x + y) as i64 {
                tree_map.insert(i, -1);
            }
            if pq_m == p as i64 {
                tree_map.insert(i, 1);
            }
            if pq_m == (p + q) as i64 {
                tree_map.insert(i, -1);
            }
        }
        let mut ans = 0;
        let mut count = 0;
        for (i, v) in tree_map {
            count += v;
            if count == 2 {
                ans = i;
                break;
            }
        }
        if ans == 0 {
            println!("{}", ans);
        } else {
            println!("infinity");
        }
    }
}

fn gcd(m: i64, n: i64) -> i64 {
    if m == 0 {
        n.abs()
    } else {
        gcd(n % m, m)
    }
}
fn lcm(m: i64, n: i64) -> i64 {
    m * n / gcd(m, n)
}
