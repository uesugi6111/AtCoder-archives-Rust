#[rustfmt::skip]
mod io_pro {
    #[macro_export] macro_rules! input{(sc=$sc:expr,$($r:tt)*)=>{input_inner!{$sc,$($r)*}};($($r:tt)*)=>{let mut sc=io_pro::Scanner::new(std::io::BufReader::new(std::io::stdin().lock()));input_inner!{sc,$($r)*}};}
    #[macro_export] macro_rules! input_inner{($sc:expr)=>{};($sc:expr,)=>{};($sc:expr,$var:ident:$t:tt$($r:tt)*)=>{let $var=read_value!($sc,$t);input_inner!{$sc $($r)*}};($sc:expr,mut $var:ident:$t:tt$($r:tt)*)=>{let mut $var=read_value!($sc,$t);input_inner!{$sc $($r)*}};}
    #[macro_export] macro_rules! read_value{($sc:expr,($($t:tt),*))=>{($(read_value!($sc,$t)),*)};($sc:expr,[$t:tt;$len:expr])=>{(0..$len).map(|_|read_value!($sc,$t)).collect::<Vec<_>>()};($sc:expr,Chars)=>{read_value!($sc,String).chars().collect::<Vec<char>>()};($sc:expr,Usize1)=>{read_value!($sc,usize)-1};($sc:expr,$t:ty)=>{$sc.next::<$t>()};}
    pub struct Scanner{s:Box<str>,input:std::str::SplitAsciiWhitespace<'static>,}
    impl Scanner{
        pub fn new<R:std::io::Read>(mut reader:R)->Self{let mut sc=Scanner{s:{let mut s=String::new();reader.read_to_string(&mut s).unwrap();s.into_boxed_str()},input:"".split_ascii_whitespace(),};let s:&'static str=unsafe{std::mem::transmute(&*sc.s)};sc.input=s.split_ascii_whitespace();sc}
        #[inline]pub fn next<T:std::str::FromStr>(&mut self)->T where T::Err:std::fmt::Debug,{self.input.next().unwrap().parse::<T>().expect("Parse error")}
    }
}
#[proconio::fastout]
fn main() {
    input!(n: usize, mut td: [(u64, u64); n]);
    td.sort_by_key(|x| x.0);
    td.push((1000000000000000001, 1000000000000000001));
    let mut td = td.into_iter().collect::<std::collections::VecDeque<_>>();
    let mut pq = std::collections::BinaryHeap::new();
    let mut count = 0;

    let mut t = 0;

    loop {
        let buff = if let Some(x) = td.front() {
            x.0
        } else {
            break;
        };

        while let Some(std::cmp::Reverse(x)) = pq.peek() {
            if t == buff {
                break;
            }
            if t <= *x {
                pq.pop();
                count += 1;
                t += 1;
            } else {
                pq.pop();
                continue;
            }
        }
        t = buff;
        loop {
            if td.front().is_none() {
                break;
            }
            let front = td.front().unwrap().clone();

            if buff != front.0 {
                break;
            }
            td.pop_front();
            pq.push(std::cmp::Reverse(front.0 + front.1));
        }
    }

    println!("{}", count);
}
