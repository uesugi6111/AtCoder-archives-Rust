use std::collections::VecDeque;
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
    input!(h: usize, w: usize, s: [Chars; h]);

    let mut memo = vec![vec![false; w]; h];

    let mut q: VecDeque<(usize, usize, usize)> = std::collections::VecDeque::new();

    let mut b_count = 0;
    for v in &s {
        for j in v {
            if *j == '#' {
                b_count += 1;
            }
        }
    }

    q.push_back((0, 0, 1));

    while let Some(p) = q.pop_front() {
        if memo[p.1][p.0] {
            continue;
        } else {
            memo[p.1][p.0] = true;
        }
        if p.0 == w - 1 && p.1 == h - 1 {
            println!("{}", h * w - b_count - p.2);
            return;
        }
        let c = s[p.1][p.0];

        if p.0 != w - 1 && c != '#' {
            q.push_back((p.0 + 1, p.1, p.2 + 1));
        }
        if p.0 != 0 && c != '#' {
            q.push_back((p.0 - 1, p.1, p.2 + 1));
        }
        if p.1 != h - 1 && c != '#' {
            q.push_back((p.0, p.1 + 1, p.2 + 1));
        }
        if p.1 != 0 && c != '#' {
            q.push_back((p.0, p.1 - 1, p.2 + 1));
        }
    }
    println!("-1");
}
