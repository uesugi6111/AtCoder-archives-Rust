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
    input!(n: usize, a: [i64; n]);
    use std::collections::HashMap;
    let mut map: HashMap<i64, Vec<usize>> = HashMap::new();

    for (i, &v) in a.iter().enumerate() {
        let v = v % 200;

        let mut add_map = HashMap::new();
        for (key, value) in map.iter() {
            if map.contains_key(&(*key + v)) {
                println!("Yes");
                let b = map.get(&(*key + v)).unwrap().clone();
                print!("{} ", b.len());
                for (k, vvv) in b.iter().enumerate() {
                    if k == b.len() - 1 {
                        println!("{}", vvv + 1);
                    } else {
                        print!("{} ", vvv + 1);
                    }
                }
                let c = value.clone();
                print!("{} ", c.len() + 1);
                for (_k, vvv) in c.iter().enumerate() {
                    print!("{} ", vvv + 1);
                }
                println!("{}", i + 1);
                return;
            }
            let mut youso = value.clone();
            youso.push(i);
            add_map.insert((*key + v) % 200, youso);
        }
        for (key, value) in add_map {
            map.insert(key, value);
        }
        if map.contains_key(&v) {
            println!("Yes");
            let b = map.get(&v).unwrap().clone();
            print!("{} ", b.len());
            for (k, vvv) in b.iter().enumerate() {
                if k == b.len() - 1 {
                    println!("{}", vvv + 1);
                } else {
                    print!("{} ", vvv + 1);
                }
            }
            print!("1 ");
            println!("{}", i + 1);
            return;
        }
        map.insert(v, vec![i]);
    }

    println!("No");
}
