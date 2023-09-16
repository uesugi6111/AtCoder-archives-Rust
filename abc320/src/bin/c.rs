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
    input!(m: usize, s: [Chars; 3]);

    let mut min = std::i64::MAX;

    for i in 0..10 {
        let mut index = 0;
        let mut riru = 0;
        while index <= 3 * m && riru < 3 {
            if s[riru][index % m].to_string() == i.to_string() {
                riru += 1;
            }

            index += 1;
        }
        if riru == 3 {
            min = min.min(index as i64);
        }
    }

    let s2 = { vec![s[0].clone(), s[2].clone(), s[1].clone()] };
    for i in 0..10 {
        let mut index = 0;
        let mut riru = 0;
        while index <= 3 * m && riru < 3 {
            if s2[riru][index % m].to_string() == i.to_string() {
                riru += 1;
            }

            index += 1;
        }
        if riru == 3 {
            min = min.min(index as i64);
        }
    }
    let s3 = { vec![s[1].clone(), s[0].clone(), s[2].clone()] };
    for i in 0..10 {
        let mut index = 0;
        let mut riru = 0;
        while index <= 3 * m && riru < 3 {
            if s3[riru][index % m].to_string() == i.to_string() {
                riru += 1;
            }

            index += 1;
        }
        if riru == 3 {
            min = min.min(index as i64);
        }
    }
    let s4 = { vec![s[1].clone(), s[2].clone(), s[0].clone()] };
    for i in 0..10 {
        let mut index = 0;
        let mut riru = 0;
        while index <= 3 * m && riru < 3 {
            if s4[riru][index % m].to_string() == i.to_string() {
                riru += 1;
            }

            index += 1;
        }
        if riru == 3 {
            min = min.min(index as i64);
        }
    }
    let s5 = { vec![s[2].clone(), s[0].clone(), s[1].clone()] };
    for i in 0..10 {
        let mut index = 0;
        let mut riru = 0;
        while index <= 3 * m && riru < 3 {
            if s5[riru][index % m].to_string() == i.to_string() {
                riru += 1;
            }

            index += 1;
        }
        if riru == 3 {
            min = min.min(index as i64);
        }
    }
    let s6 = { vec![s[2].clone(), s[1].clone(), s[0].clone()] };
    for i in 0..10 {
        let mut index = 0;
        let mut riru = 0;
        while index <= 3 * m && riru < 3 {
            if s6[riru][index % m].to_string() == i.to_string() {
                riru += 1;
            }

            index += 1;
        }
        if riru == 3 {
            min = min.min(index as i64);
        }
    }
    println!("{}", if min == std::i64::MAX { -1 } else { min - 1 });
}
