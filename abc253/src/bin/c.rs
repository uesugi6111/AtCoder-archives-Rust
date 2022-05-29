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

    let mut bt = bt::BinaryTrie::new();
    for _ in 0..q {
        input!(sc = sc, t: i64);
        if t == 1 {
            input!(sc = sc, x: u32);
            bt.insert(x);
        } else if t == 2 {
            input!(sc = sc, x: u32, c: u64);
            bt.erase_n(x, c.min(bt.count(x)));
        } else {
            println!("{}", bt.max().unwrap() - bt.min().unwrap());
        }
    }
}

mod bt {
    //! BinaryTire

    #[derive(Debug, Default, Clone, PartialEq, Eq)]
    struct Node {
        children: [Option<usize>; 2],
        count: u64,
    }
    impl Node {
        #[inline]
        fn new() -> Self {
            Self {
                children: [None; 2],
                count: 0,
            }
        }
    }

    #[derive(Debug, Default, Clone, PartialEq, Eq)]
    pub struct BinaryTrie {
        nodes: Vec<Node>,
    }
    impl BinaryTrie {
        /// 構築
        #[inline]
        pub fn new() -> Self {
            Self {
                nodes: vec![Node::new()],
            }
        }

        /// 値の挿入
        #[inline]
        pub fn insert(&mut self, x: u32) -> u64 {
            self.insert_n(x, 1)
        }
        #[inline]
        pub fn insert_n(&mut self, x: u32, n: u64) -> u64 {
            if n == 0 {
                return 0;
            }
            let mut node_index = 0;
            for i in (0..32).rev() {
                self.nodes[node_index].count += n;

                node_index = match self.nodes[node_index].children[(x >> i & 1) as usize] {
                    Some(i) => i,
                    None => {
                        self.nodes.push(Node::new());
                        self.nodes[node_index].children[(x >> i & 1) as usize] =
                            Some(self.nodes.len() - 1);
                        self.nodes.len() - 1
                    }
                };
            }
            self.nodes[node_index].count += n;
            self.nodes[node_index].count
        }

        /// xのカウント
        #[inline]
        pub fn count(&self, x: u32) -> u64 {
            let mut node_index = Some(0);

            for i in (0..32).rev() {
                if node_index.is_none() {
                    return 0;
                }
                node_index = self.nodes[node_index.unwrap()].children[(x >> i & 1) as usize];
            }
            if node_index.is_none() {
                return 0;
            }
            self.nodes[node_index.unwrap()].count
        }

        /// x 未満の値のカウント
        #[inline]
        pub fn count_less(&self, x: u32) -> u64 {
            self.inner_count_than(x, 1)
        }

        /// x を超える値のカウント
        #[inline]
        pub fn count_more(&self, x: u32) -> u64 {
            self.inner_count_than(x, 0)
        }
        #[inline]
        fn inner_count_than(&self, x: u32, bit: u32) -> u64 {
            let mut node_index = Some(0);

            let mut count = 0;
            for i in (0..32).rev() {
                if node_index.is_none() {
                    break;
                }
                if (x >> i & 1) == bit {
                    count += match self.nodes[node_index.unwrap()].children[(bit ^ 1) as usize] {
                        Some(i) => self.nodes[i].count,
                        None => 0,
                    }
                }
                node_index = self.nodes[node_index.unwrap()].children[(x >> i & 1) as usize];
            }
            count
        }

        /// 値の削除
        #[inline]
        pub fn erase(&mut self, x: u32) -> Option<()> {
            if 1 > self.count(x) {
                return None;
            }
            self.inner_erase(x, 1)
        }
        #[inline]
        pub fn erase_n(&mut self, x: u32, n: u64) -> Option<()> {
            if n > self.count(x) {
                return None;
            }
            self.inner_erase(x, n)
        }

        /// 値をすべて削除
        #[inline]
        pub fn erase_all(&mut self, x: u32) -> Option<()> {
            let erase_count = self.count(x);
            if erase_count == 0 {
                return None;
            }
            self.inner_erase(x, erase_count)
        }

        /// 値を削除
        /// 内部関数
        #[inline]
        fn inner_erase(&mut self, x: u32, erase_count: u64) -> Option<()> {
            let mut node_index = Some(0);
            for i in (0..32).rev() {
                self.nodes[node_index?].count -= erase_count;
                node_index = self.nodes[node_index?].children[(x >> i & 1) as usize];
            }
            self.nodes[node_index?].count -= erase_count;

            Some(())
        }

        /// xor 後の最小値を求める
        #[inline]
        pub fn xor_min(&self, x: u32) -> Option<u32> {
            let mut ans = 0;

            let mut node_index = Some(0);
            for i in (0..32).rev() {
                let bit = {
                    let mut buff = (x >> i & 1) as usize;
                    if self.nodes[node_index.unwrap()].children[buff]
                        .filter(|&index| self.nodes[index].count > 0)
                        .is_none()
                    {
                        buff ^= 1;
                    }
                    buff
                };
                ans ^= (bit as u32) << i;
                node_index = self.nodes[node_index.unwrap()].children[bit];
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
            let max = self.size();
            self.xth_element(max)
        }
        #[inline]
        pub fn size(&self) -> u64 {
            self.nodes[0].count
        }
        #[inline]
        pub fn xth_element(&self, xth: u64) -> Option<u32> {
            let mut x = xth;
            let mut ans = 0;
            let mut node_index = Some(0);

            for i in (0..32).rev() {
                let count = if let Some(i) = self.nodes[node_index.unwrap()].children[0] {
                    self.nodes[i].count
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
                node_index = self.nodes[node_index.unwrap()].children[bit];
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
            assert_eq!(b.size(), 1);

            b.insert(7);
            b.insert(7);
            assert_eq!(b.size(), 3);
            assert_eq!(b.xth_element(1).unwrap(), 6);
            assert_eq!(b.xth_element(2).unwrap(), 7);
            assert_eq!(b.xth_element(3).unwrap(), 7);
            b.erase(7);
            b.erase(7);
            assert_eq!(b.count(2), 0);
            assert_eq!(b.count(3), 0);
            assert_eq!(b.count(4), 0);
            assert_eq!(b.count(5), 0);
            assert_eq!(b.count(8), 0);
            assert_eq!(b.size(), 1);
            assert_eq!(b.erase(10), None);
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
        fn test_count_than() {
            let mut b = BinaryTrie::new();

            for i in 0..1000 {
                b.insert(i);
                assert_eq!(b.count_less(i), i as u64);
            }

            assert_eq!(b.min().unwrap(), 0);
            assert_eq!(b.max().unwrap(), 999);
            for i in 0..1000 {
                assert_eq!(b.count_more(i), 999 - i as u64);
            }
            assert_eq!(b.count_less(std::u32::MAX), 1000);
            assert_eq!(b.count_more(std::u32::MIN), 999);
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
            assert_eq!(b.count(6), 1);
            assert_eq!(b.count(7), 0);
        }

        #[test]
        fn q() {
            use crate::other::xorshift::XorShift;
            let mut xs = XorShift::new();
            let mut b = BinaryTrie::new();
            b.insert(0);
            let mut ans = vec![];
            for i in 0..200_000 {
                match xs.next().unwrap() % 3 {
                    0 => {
                        b.insert(xs.next().unwrap() as u32 % std::u32::MAX);
                    }
                    1 => {
                        b.erase_all(xs.next().unwrap() as u32 % std::u32::MAX);
                    }
                    _ => ans.push(
                        b.xor_min(xs.next().unwrap() as u32 % std::u32::MAX)
                            .unwrap_or_else(|| panic!()),
                    ),
                }

                b.xor_min(i);
            }
        }
    }
}
