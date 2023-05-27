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
    input!(n: usize, uv: [(usize, usize); n - 1]);

    let mut e = vec![vec![]; n];

    for (u, v) in uv {
        e[u - 1].push(v - 1);
        e[v - 1].push(u - 1);
    }
    let mut color = vec![4; n];

    let buff = e.iter().position(|x| x.len() == 1).unwrap();
    f(&e, 0, &mut color, buff);

    let mut ans = vec![];
    for (i, v) in color.iter().enumerate() {
        if *v != 1 {
            continue;
        }
        ans.push(e[i].len());
    }

    ans.sort_unstable();
    for i in 0..ans.len() {
        print!("{} ", ans[i]);
    }
}

fn f(e: &[Vec<usize>], depth: usize, color: &mut Vec<usize>, v: usize) {
    if color[v] != 4 {
        return;
    }

    color[v] = depth % 3;

    for &i in &e[v] {
        f(e, depth + 1, color, i);
    }
}
