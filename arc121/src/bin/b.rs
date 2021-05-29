#[rustfmt::skip]
mod io_pro {
    #[macro_export] macro_rules! input{(sc=$sc:expr,$($r:tt)*)=>{input_inner!{$sc,$($r)*}};($($r:tt)*)=>{let mut sc=io_pro::Scanner::new(std::io::stdin().lock());input_inner!{sc,$($r)*}};}
    #[macro_export] macro_rules! input_inner{($sc:expr)=>{};($sc:expr,)=>{};($sc:expr,$var:ident:$t:tt$($r:tt)*)=>{let $var=read_value!($sc,$t);input_inner!{$sc $($r)*}};}
    #[macro_export] macro_rules! read_value{($sc:expr,($($t:tt),*))=>{($(read_value!($sc,$t)),*)};($sc:expr,[$t:tt;$len:expr])=>{(0..$len).map(|_|read_value!($sc,$t)).collect::<Vec<_>>()};($sc:expr,Chars)=>{read_value!($sc,String).chars().collect::<Vec<char>>()};($sc:expr,Usize1)=>{read_value!($sc,usize)-1};($sc:expr,$t:ty)=>{$sc.next::<$t>()};}
    pub struct Scanner{s:Box<str>,input:std::iter::Peekable<std::str::SplitAsciiWhitespace<'static>>,}
    impl Scanner{
        pub fn new<R:std::io::Read>(mut reader:R)->Self{let mut sc=Scanner{s:{let mut s=String::new();reader.read_to_string(&mut s).unwrap();s.into_boxed_str()},input:"".split_ascii_whitespace().peekable(),};let s:&'static str=unsafe{std::mem::transmute(&*sc.s)};sc.input=s.split_ascii_whitespace().peekable();sc}
        #[inline]pub fn next<T:std::str::FromStr>(&mut self)->T where T::Err:std::fmt::Debug,{self.input.next().unwrap().parse::<T>().expect("Parse error")}
    }
}
#[proconio::fastout]
fn main() {
    input!(n: usize, ac: [(i64, char); 2 * n]);
    let mut ac = ac;
    ac.sort_unstable();
    let (mut b_r, mut r_g, mut g_b) = (std::i64::MAX, std::i64::MAX, std::i64::MAX);
    let (mut r_i, mut g_i, mut b_i) = (-1, -1, -1);
    let mut count = vec![0i64; 3];
    for (i, (a, c)) in ac.iter().enumerate() {
        match c {
            'R' => {
                if b_i != -1 {
                    b_r = b_r.min((ac[b_i as usize].0 - *a).abs());
                }
                if g_i != -1 {
                    r_g = r_g.min((ac[g_i as usize].0 - *a).abs());
                }
                r_i = i as i32;
                count[0] += 1;
            }
            'G' => {
                if b_i != -1 {
                    g_b = g_b.min((ac[b_i as usize].0 - *a).abs());
                }
                if r_i != -1 {
                    r_g = r_g.min((ac[r_i as usize].0 - *a).abs());
                }
                g_i = i as i32;
                count[1] += 1;
            }
            'B' => {
                if r_i != -1 {
                    b_r = b_r.min((ac[r_i as usize].0 - *a).abs());
                }
                if g_i != -1 {
                    g_b = g_b.min((ac[g_i as usize].0 - *a).abs());
                }
                b_i = i as i32;
                count[2] += 1;
            }
            _ => {}
        }
    }

    println!(
        "{}",
        if count[0] % 2 == 0 {
            if count[1] % 2 == 0 {
                0
            } else {
                g_b
            }
        } else if count[1] % 2 == 0 {
            b_r
        } else {
            r_g
        }
    );
}
