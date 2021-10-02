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
    input!(n: usize, a: [u32; n]);
    let mut bt = BinaryTrie::new();
    let mut ans = 0;
    let mut map = std::collections::HashMap::new();
    for i in 0..n {
        ans += bt.count_lower_than(a[i]);
        if let Some(v) = map.get(&a[i]) {
            ans += *v;
        }
        *map.entry(a[i]).or_insert_with(vec![]).push(i);
        ans %= 998244353;
        bt.insert(a[i]);
    }

    println!("{}", ans);
}
pub struct BinaryTrie {
    cnt: usize,
    lch: Option<Box<BinaryTrie>>,
    rch: Option<Box<BinaryTrie>>,
}

impl BinaryTrie {
    pub fn new() -> Self {
        Self {
            cnt: 0,
            lch: None,
            rch: None,
        }
    }

    pub fn size(&self) -> usize {
        self.cnt
    }

    pub fn contains(&mut self, mut val: u32) -> bool {
        let mut node = self;
        val = val.reverse_bits();
        for _ in 0..32 {
            node = if val & 1 == 0 {
                if node.lch.is_none() {
                    return false;
                }
                node.lch.as_deref_mut().unwrap()
            } else {
                if node.rch.is_none() {
                    return false;
                }
                node.rch.as_deref_mut().unwrap()
            };
            val >>= 1;
        }
        true
    }

    pub fn insert(&mut self, mut val: u32) {
        if self.contains(val) {
            return;
        }
        self.cnt += 1;
        let mut node = self;
        val = val.reverse_bits();
        for _ in 0..32 {
            node = if val & 1 == 0 {
                if node.lch.is_none() {
                    node.lch = Some(Box::new(Self::new()));
                }
                node.lch.as_deref_mut().unwrap()
            } else {
                if node.rch.is_none() {
                    node.rch = Some(Box::new(Self::new()));
                }
                node.rch.as_deref_mut().unwrap()
            };
            node.cnt += 1;
            val >>= 1;
        }
    }

    pub fn erase(&mut self, mut val: u32) {
        if !self.contains(val) {
            return;
        }
        self.cnt -= 1;
        let mut node = self;
        val = val.reverse_bits();
        for _ in 0..32 {
            node = if val & 1 == 0 {
                assert!(node.lch.is_some());
                if node.lch.as_ref().unwrap().cnt == 1 {
                    node.lch = None;
                    return;
                }
                node.lch.as_deref_mut().unwrap()
            } else {
                assert!(node.rch.is_some());
                if node.rch.as_ref().unwrap().cnt == 1 {
                    node.rch = None;
                    return;
                }
                node.rch.as_deref_mut().unwrap()
            };
            node.cnt -= 1;
            val >>= 1;
        }
    }

    pub fn xor_min(&mut self, mut val: u32) -> u32 {
        let mut node = self;
        val = val.reverse_bits();
        let mut res = 0;
        for _ in 0..32 {
            res <<= 1;
            node = if val & 1 == 0 && node.lch.is_some() || node.rch.is_none() {
                node.lch.as_deref_mut().unwrap()
            } else {
                res |= 1;
                node.rch.as_deref_mut().unwrap()
            };
            val >>= 1;
        }
        res
    }

    pub fn max(&mut self) -> u32 {
        self.xor_min(!0)
    }

    pub fn min(&mut self) -> u32 {
        self.xor_min(0)
    }

    pub fn count_lower_than(&mut self, mut val: u32) -> usize {
        let mut node = self;
        val = val.reverse_bits();
        let mut res = 0;
        for _ in 0..32 {
            if val & 1 == 1 && node.lch.is_some() {
                res += node.lch.as_deref().unwrap().cnt;
            }
            node = if val & 1 == 0 {
                if node.lch.is_none() {
                    return res;
                }
                node.lch.as_deref_mut().unwrap()
            } else {
                if node.rch.is_none() {
                    return res;
                }
                node.rch.as_deref_mut().unwrap()
            };
            val >>= 1;
        }
        res
    }
}
