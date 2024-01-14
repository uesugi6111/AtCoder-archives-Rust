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
//#[proconio::fastout]
fn main() {
    input!(n: usize, t: Chars, s: [Chars; n]);
    let mut ans = std::collections::BTreeSet::new();
    let tt = t.iter().collect::<String>();

    let s1 = s
        .iter()
        .enumerate()
        .filter(|x| x.1.len() == t.len())
        .map(|x| (x.0, x.1.iter().collect::<String>()))
        .collect::<Vec<_>>();
    for i in 0..s1.len() {
        if s1[i].1 == tt {
            ans.insert(s1[i].0);
        }
    }

    let s2 = s
        .iter()
        .enumerate()
        .filter(|x| x.1.len() == t.len() - 1)
        .collect::<Vec<_>>();

    'aaa: for i in 0..s2.len() {
        let mut index = 0;
        let mut skipped = false;
        for j in 0..t.len() {
            if j == t.len() - 1 && !skipped {
                continue;
            }
            if t[j] == s2[i].1[index] {
                index += 1;
            } else {
                if skipped {
                    continue 'aaa;
                }
                skipped = true;
            }
        }
        ans.insert(s2[i].0);
    }

    let s3 = s
        .iter()
        .enumerate()
        .filter(|x| x.1.len() == t.len() + 1)
        .collect::<Vec<_>>();

    'aaa: for i in 0..s3.len() {
        let mut index = 0;
        let mut skipped = false;
        for j in 0..s3[i].1.len() {
            if j == s3[i].1.len() - 1 && !skipped {
                continue;
            }
            if t[index] == s3[i].1[j] {
                index += 1;
            } else {
                if skipped {
                    continue 'aaa;
                }
                skipped = true;
            }
        }
        ans.insert(s3[i].0);
    }

    let s4 = s
        .iter()
        .enumerate()
        .filter(|x| x.1.len() == t.len())
        .collect::<Vec<_>>();

    'aaa: for i in 0..s4.len() {
        let mut skipped = false;
        for j in 0..t.len() {
            if t[j] == s4[i].1[j] {
            } else {
                if skipped {
                    continue 'aaa;
                }
                skipped = true;
            }
        }
        ans.insert(s4[i].0);
    }
    println!("{}", ans.len());
    for i in ans.iter() {
        print!("{} ", i + 1);
    }
}
