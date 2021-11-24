#[rustfmt::skip]
mod io_pro {
    #[macro_export] macro_rules! input{(sc=$sc:expr,$($r:tt)*)=>{input_inner!{$sc,$($r)*}};($($r:tt)*)=>{let mut sc=io_pro::Scanner::new(std::io::stdin().lock());input_inner!{sc,$($r)*}};}
    #[macro_export] macro_rules! input_inner{($sc:expr)=>{};($sc:expr,)=>{};($sc:expr,$var:ident:$t:tt$($r:tt)*)=>{let $var=read_value!($sc,$t);input_inner!{$sc $($r)*}};($sc:expr,mut $var:ident:$t:tt$($r:tt)*)=>{let mut $var=read_value!($sc,$t);input_inner!{$sc $($r)*}};}
    #[macro_export] macro_rules! read_value{($sc:expr,($($t:tt),*))=>{($(read_value!($sc,$t)),*)};($sc:expr,[$t:tt;$len:expr])=>{(0..$len).map(|_|read_value!($sc,$t)).collect::<Vec<_>>()};($sc:expr,Chars)=>{read_value!($sc,String).chars().collect::<Vec<char>>()};($sc:expr,Usize1)=>{read_value!($sc,usize)-1};($sc:expr,$t:ty)=>{$sc.next::<$t>()};}
    pub struct Scanner{s:Box<str>,input:std::iter::Peekable<std::str::SplitAsciiWhitespace<'static>>,}
    impl Scanner{
        pub fn new<R:std::io::Read>(mut reader:R)->Self{let mut sc=Scanner{s:{let mut s=String::new();reader.read_to_string(&mut s).unwrap();s.into_boxed_str()},input:"".split_ascii_whitespace().peekable(),};let s:&'static str=unsafe{std::mem::transmute(&*sc.s)};sc.input=s.split_ascii_whitespace().peekable();sc}
        #[inline]pub fn next<T:std::str::FromStr>(&mut self)->T where T::Err:std::fmt::Debug,{self.input.next().unwrap().parse::<T>().expect("Parse error")}
    }
}
#[proconio::fastout]
fn main() {
    let mut sc = io_pro::Scanner::new(std::io::stdin().lock());
    input!(sc = sc, q: usize);
    let mut tree = std::collections::BTreeMap::new();
    tree.insert(0, -1);

    let mut v: Vec<std::ops::Range<_>> = vec![0..1];
    for _ in 0..q {
        input!(sc = sc, t: i64, x: i64);

        if t == 1 {
            let mut buff = x % 1048576;

            let mut hukumu = false;

            for i in 0..v.len() {
                if v[i].contains(&((buff + 1048576 - 1) % 1048576)) {
                    buff = v[i].end;
                    if buff == 1048576 {
                        buff = v[0].end;
                        v[0] = 0..buff + 1;
                        hukumu = true;
                        break;
                    }

                    v[i] = v[i].start..buff + 1;
                    hukumu = true;
                    break;
                }
            }

            if !hukumu {
                v.push(buff..buff + 1);
            }

            tree.insert(buff, x);
        } else {
            println!("{}", tree.get(&(x % 1048576)).unwrap_or(&-1));
        }
    }
}
