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
    input!(n: usize, x: [i64; n]);
    let p = vec![1, 2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47];
    let mut min = std::i64::MAX;
    for pp in p {
        let mut set = std::collections::HashSet::new();
        set.insert(&pp);

        let mut v = vec![];
        let aaa: Vec<_> = x.iter().map(|&y| enum_divisors(y)).collect();

        for i in 0..n {
            v.push((*aaa[i].iter().min().unwrap(), aaa[i].clone()));
        }
        v.sort_by_key(|x| x.0);

        for (i, div) in v.iter().rev() {
            let mut is_ok = false;
            for j in div.iter() {
                if set.contains(j) {
                    is_ok = true;
                    break;
                }
            }
            if !is_ok {
                set.insert(i);
            }
        }

        let mut ans = 1;
        for i in set {
            ans *= i;
        }
        min = min.min(ans);
    }

    println!("{}", min);
}

pub fn enum_divisors(n: i64) -> std::collections::HashSet<i64> {
    let mut res = std::collections::HashSet::new();
    for i in 1..=(n as f64).sqrt() as i64 {
        if n % i != 0 {
            continue;
        }
        res.insert(i);
        if i.pow(2) != n {
            res.insert(n / i);
        }
    }
    res.remove(&1);

    res
}
