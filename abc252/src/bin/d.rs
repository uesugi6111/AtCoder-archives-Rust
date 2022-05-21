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

    let mut v = vec![0_u64; n];

    let mut bt = bt::BinaryTrie::new();
    for (_, &value) in a.iter().enumerate() {
        bt.insert(value);
    }
    for (i, &value) in a.iter().enumerate() {
        v[i] = bt.count_less(value).unwrap_or(0);
    }
    let mut sum = 0;
    let mut bt2 = bt::BinaryTrie::new();
    for (i, &value) in a.iter().enumerate() {
        bt2.insert_n(value, v[i]);
    }
    for (i, &value) in a.iter().enumerate() {
        sum += bt2.count_less(value).unwrap_or(0);
    }

    println!("{}", sum);
}

mod bt {

    #[derive(Debug, Default, Clone, PartialEq, Eq)]
    struct Node {
        children: Vec<Option<Node>>,
        count: u64,
    }
    impl Node {
        #[inline]
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
        #[inline]
        pub const fn new() -> Self {
            Self { nodes: None }
        }

        /// 値の挿入
        #[inline]
        pub fn insert(&mut self, x: u32) -> Option<()> {
            let mut node = self.nodes.get_or_insert_with(Node::new);

            for i in (0..32).rev() {
                node.count += 1;
                node = node.children[(x >> i & 1) as usize].get_or_insert_with(Node::new);
            }
            node.count += 1;
            Some(())
        }
        #[inline]
        pub fn insert_n(&mut self, x: u32, n: u64) -> Option<()> {
            if n == 0 {
                return None;
            }
            let mut node = self.nodes.get_or_insert_with(Node::new);

            for i in (0..32).rev() {
                node.count += n;
                node = node.children[(x >> i & 1) as usize].get_or_insert_with(Node::new);
            }
            node.count += n;
            Some(())
        }

        /// 値のカウント
        #[inline]
        pub fn count(&self, x: u32) -> Option<u64> {
            let mut node = &self.nodes;

            for i in (0..32).rev() {
                node = &node.as_ref()?.children[(x >> i & 1) as usize];
            }
            Some(node.as_ref()?.count)
        }

        #[inline]
        pub fn count_less(&self, x: u32) -> Option<u64> {
            let mut node = &self.nodes;

            let mut count = 0;
            for i in (0..32).rev() {
                if node.as_ref().is_none() {
                    break;
                }
                if (x >> i & 1) == 1 {
                    count += match &node.as_ref()?.children[0].as_ref() {
                        Some(v) => v.count,
                        None => 0,
                    }
                }

                node = &node.as_ref()?.children[(x >> i & 1) as usize];
            }
            Some(count)
        }

        /// 値の削除
        #[inline]
        pub fn erase(&mut self, x: u32) -> Option<()> {
            self.count(x)?;
            self.erase_inner(x, 1)
        }

        /// 値をすべて削除
        #[inline]
        pub fn erase_all(&mut self, x: u32) -> Option<()> {
            let erase_count = self.count(x)?;
            self.erase_inner(x, erase_count)
        }

        /// 値を削除
        /// 内部関数
        #[inline]
        fn erase_inner(&mut self, x: u32, erase_count: u64) -> Option<()> {
            let mut node = &mut self.nodes;
            for i in (0..32).rev() {
                if node.as_ref()?.count == erase_count {
                    *node = None;
                    return Some(());
                } else {
                    node.as_mut()?.count -= erase_count;
                }
                node = &mut node.as_mut()?.children[(x >> i & 1) as usize];
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
                    if node.children[buff].is_none() {
                        buff ^= 1;
                    }
                    buff
                };
                ans ^= (bit as u32) << i;
                node = node.children[bit].as_ref()?;
            }
            Some(ans ^ x)
        }

        /// 最小値を求める
        #[inline]
        pub fn min(&self) -> Option<u32> {
            self.xth_element(1)
        }

        /// 最大値を求める
        #[inline]
        pub fn max(&self) -> Option<u32> {
            let max = self.size()?;
            self.xth_element(max)
        }
        #[inline]
        pub fn size(&self) -> Option<u64> {
            Some(self.nodes.as_ref()?.count)
        }
        #[inline]
        pub fn xth_element(&self, xth: u64) -> Option<u32> {
            let mut x = xth;
            let mut ans = 0;
            let mut node = self.nodes.as_ref()?;

            for i in (0..32).rev() {
                let count = if let Some(node) = node.children[0].as_ref() {
                    node.count
                } else {
                    0
                };

                let bit = if count >= x {
                    0
                } else {
                    x -= count;
                    1
                };
                ans ^= (bit as u32) << i;
                node = node.children[bit].as_ref()?;
            }

            Some(ans)
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn bt() {
            let mut b = BinaryTrie::new();
            b.insert(6);
            assert_eq!(b.size().unwrap(), 1);

            let a = b.clone();
            b.insert(7);
            b.insert(7);
            assert_eq!(b.size().unwrap(), 3);
            assert_eq!(b.xth_element(1).unwrap(), 6);
            assert_eq!(b.xth_element(2).unwrap(), 7);
            assert_eq!(b.xth_element(3).unwrap(), 7);
            b.erase(7);
            b.erase(7);
            assert_eq!(b.size().unwrap(), 1);
            assert_eq!(b.erase(10), None);
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
        fn bttt() {
            let mut b = BinaryTrie::new();

            for i in [1, 1, 3, 4] {
                b.insert(i);
            }

            assert_eq!(b.min().unwrap(), 1);

            assert_eq!(b.count_less(0).unwrap(), 0);
            assert_eq!(b.count_less(3).unwrap(), 2);
            assert_eq!(b.count_less(4).unwrap(), 3);
            assert_eq!(b.count_less(std::u32::MAX).unwrap(), 4);
        }

        #[test]
        fn library_checker() {
            let mut b = BinaryTrie::new();
            let query = vec![(0, 6), (0, 7), (2, 5), (1, 7), (1, 10), (2, 7)];
            let mut ans = vec![];
            query.iter().for_each(|&(p, x)| match p {
                0 => {
                    b.insert(x);
                }
                1 => {
                    b.erase_all(x);
                }
                _ => ans.push(b.xor_min(x).unwrap_or_else(|| panic!("{}", x.to_string()))),
            });

            assert_eq!(vec![2, 1], ans);
        }
    }
}
