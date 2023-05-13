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
    input!(s: Chars, t: Chars);
    let mut btree_map_s = std::collections::BTreeMap::new();
    let mut btree_map_t = std::collections::BTreeMap::new();

    for i in 0..s.len() {
        *btree_map_s.entry(s[i]).or_default() += 1;
        *btree_map_t.entry(t[i]).or_default() += 1;
    }

    let set = vec!['a', 't', 'c', 'o', 'd', 'e', 'r']
        .iter()
        .copied()
        .collect::<std::collections::HashSet<char>>();

    let mut at_s = 0;
    let mut at_t = 0;
    for k in &[
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
        's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '@',
    ] {
        let tt = *btree_map_t.get(k).unwrap_or(&0);
        let ss = *btree_map_s.get(k).unwrap_or(&0);
        if set.contains(k) {
            if tt < ss {
                at_t += ss - tt;
            } else {
                at_s += tt - ss;
            }
        } else if *k == '@' {
            if ss - at_s < 0 || ss - at_s != tt - at_t {
                println!("No");
                return;
            }
        } else if tt != ss {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
