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
use std::io::{stdout, BufWriter, Write};
fn main() {
    input!(q: usize, query: [(i32, u32); q]);

    let out = stdout();
    let mut out_lock = BufWriter::new(out.lock());

    let mut b = bt::BinaryTrie::new();
    query.iter().for_each(|&(p, x)| match p {
        0 => b.insert(x),
        1 => {
            b.erase_all(x);
        }
        _ => {
            writeln!(
                &mut out_lock,
                "{}",
                b.xor_min(x).unwrap_or_else(|| panic!(x.to_string()))
            )
            .ok();
        }
    });
}

mod bt {
    //! BinaryTire

    #[derive(Debug, Default, Clone, PartialEq, Eq)]
    struct Node {
        children: Vec<Option<Node>>,
        count: u32,
    }
    impl Node {
        fn new() -> Self {
            Self {
                children: vec![None; 2],
                count: 0,
            }
        }
    }

    #[derive(Debug, Default, Clone, PartialEq, Eq)]
    pub struct BinaryTrie {
        nodes: Option<Node>,
    }
    impl BinaryTrie {
        /// 構築
        pub const fn new() -> Self {
            Self { nodes: None }
        }

        /// 値の挿入
        #[inline]
        pub fn insert(&mut self, x: u32) {
            if self.nodes.is_none() {
                self.nodes = Some(Node::new());
            }
            let mut node = self.nodes.as_mut().unwrap();
            for i in (0..32).rev() {
                node.count += 1;
                let bit = (x >> i & 1) as usize;
                if unsafe { node.children.get_unchecked(bit) }.is_none() {
                    *unsafe { node.children.get_unchecked_mut(bit) } = Some(Node::new());
                }
                node = unsafe { node.children.get_unchecked_mut(bit) }
                    .as_mut()
                    .unwrap();
            }
            node.count += 1;
        }

        /// 値のカウント
        #[inline]
        pub fn count(&self, x: u32) -> Option<u32> {
            let mut node = &self.nodes;

            for i in (0..32).rev() {
                node = unsafe { node.as_ref()?.children.get_unchecked((x >> i & 1) as usize) };
            }
            Some(node.as_ref()?.count)
        }

        /// 値の削除
        #[inline]
        pub fn erase(&mut self, x: u32) -> Option<()> {
            self.count(x)?;

            self.erase_inner(x, 1)?;

            Some(())
        }

        /// 値をすべて削除
        #[inline]
        pub fn erase_all(&mut self, x: u32) -> Option<()> {
            let erase_count = self.count(x)?;

            self.erase_inner(x, erase_count)?;

            Some(())
        }

        /// 値を削除
        /// 内部関数
        fn erase_inner(&mut self, x: u32, erase_count: u32) -> Option<()> {
            let mut node = &mut self.nodes;
            for i in (0..32).rev() {
                if node.as_ref()?.count == erase_count {
                    *node = None;
                    return Some(());
                } else {
                    node.as_mut()?.count -= erase_count;
                }
                node = unsafe {
                    node.as_mut()?
                        .children
                        .get_unchecked_mut((x >> i & 1) as usize)
                };
            }
            if node.as_ref()?.count == erase_count {
                *node = None;
            } else {
                node.as_mut()?.count -= erase_count;
            }

            Some(())
        }

        /// xor 後の最小値を求める
        #[inline]
        pub fn xor_min(&self, x: u32) -> Option<u32> {
            let mut ans = 0;
            let mut node = self.nodes.as_ref()?;
            for i in (0..32).rev() {
                let bit = {
                    let mut buff = (x >> i & 1) as usize;
                    if unsafe { node.children.get_unchecked(buff) }.is_none() {
                        buff ^= 1;
                    }
                    buff
                };
                ans ^= (bit as u32) << i;
                node = unsafe { node.children.get_unchecked(bit) }
                    .as_ref()
                    .unwrap();
            }
            Some(ans ^ x)
        }

        /// 最小値を求める
        #[inline]
        pub fn min(&self) -> Option<u32> {
            let mut ans = 0;
            let mut node = self.nodes.as_ref()?;
            for i in (0..32).rev() {
                let bit = if node.children[0].is_none() { 1 } else { 0 };
                ans ^= (bit as u32) << i;
                node = unsafe { node.children.get_unchecked(bit) }
                    .as_ref()
                    .unwrap();
            }
            Some(ans)
        }

        /// 最大値を求める
        #[inline]
        pub fn max(&self) -> Option<u32> {
            let mut ans = 0;
            let mut node = self.nodes.as_ref()?;
            for i in (0..32).rev() {
                let bit = if node.children[1].is_none() { 0 } else { 1 };
                ans ^= (bit as u32) << i;
                node = unsafe { node.children.get_unchecked(bit) }
                    .as_ref()
                    .unwrap();
            }
            Some(ans)
        }
    }

    #[cfg(test)]
    mod test {

        use super::*;
        #[test]
        fn bt() {
            let mut b = BinaryTrie::new();
            b.insert(6);

            let a = b.clone();
            b.insert(7);
            b.insert(7);
            b.erase(7);
            b.erase(7);
            b.erase(10);
            assert_eq!(a.nodes, b.nodes);
        }
        #[test]
        fn btt() {
            let mut b = BinaryTrie::new();
            let n = 2u32.pow(30);
            for i in 0..100 {
                b.insert(n + i);
            }
            for i in 0..99 {
                b.erase(n + i);
                assert_eq!(b.min().unwrap(), n + i + 1);
            }
        }

        #[test]
        fn library_checker() {
            let mut b = BinaryTrie::new();
            let query = vec![(0, 6), (0, 7), (2, 5), (1, 7), (1, 10), (2, 7)];
            let mut ans = vec![];
            query.iter().for_each(|&(p, x)| match p {
                0 => b.insert(x),
                1 => {
                    b.erase_all(x);
                }
                _ => ans.push(b.xor_min(x).unwrap_or_else(|| panic!("{}", x.to_string()))),
            });

            assert_eq!(vec![2, 1], ans);
        }
    }
}
