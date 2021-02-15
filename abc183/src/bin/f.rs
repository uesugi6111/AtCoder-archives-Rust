#[rustfmt::skip]
mod fast_input {
    #[macro_export] macro_rules! input{(sc=$sc:expr,$($r:tt)*)=>{input_inner!{$sc,$($r)*}};($($r:tt)*)=>{let mut sc=fast_input::Scanner::new(std::io::stdin().lock(),4096);input_inner!{sc,$($r)*}};}
    #[macro_export] macro_rules! input_inner{($sc:expr)=>{};($sc:expr,)=>{};($sc:expr,$var:ident:$t:tt$($r:tt)*)=>{let $var=read_value!($sc,$t);input_inner!{$sc $($r)*}};}
    #[macro_export] macro_rules! read_value{($sc:expr,($($t:tt),*))=>{($(read_value!($sc,$t)),*)};($sc:expr,[$t:tt;$len:expr])=>{(0..$len).map(|_|read_value!($sc,$t)).collect::<Vec<_>>()};($sc:expr,Chars)=>{read_value!($sc,String).chars().collect::<Vec<char>>()};($sc:expr,Usize1)=>{read_value!($sc,usize)-1};($sc:expr,$t:ty)=>{$sc.next::<$t>()};}
    pub struct Scanner {buf:Vec<u8>,pos: usize,}
    impl Scanner {
        pub fn new<R: std::io::Read>(mut reader: R, estimated: usize) -> Self {
            let mut buf = Vec::with_capacity(estimated);let _=std::io::copy(&mut reader,&mut buf).unwrap();if buf.last()!=Some(&b'\n'){panic!("{}", 0);}
            Scanner { buf, pos: 0 }
        }
        #[inline]
        pub fn next<T: std::str::FromStr>(&mut self) -> T where T::Err: std::fmt::Debug,{
            let mut start=None;loop{match(self.buf[self.pos],start.is_some()){(b' ',true)|(b'\n', true)=>break,(_, true)|(b' ', false)|(b'\n',false)=>self.pos+=1,(_, false)=>start=Some(self.pos),}}let target=&self.buf[start.unwrap()..self.pos];
            unsafe { std::str::from_utf8_unchecked(target) }.parse().unwrap()
        }
    }
}
#[proconio::fastout]
fn main() {
    let mut sc = fast_input::Scanner::new(std::io::stdin().lock(), 4096);
    input!(sc = sc, n: usize);
    input!(
        sc = sc,
        q: usize,
        c: [usize; n],
        query: [(usize, usize, usize); q]
    );
    let mut dsu = dsu::Dsu::new(c);

    for qq in query {
        if qq.0 == 1 {
            dsu.unite(qq.1 - 1, qq.2 - 1);
        } else {
            println!("{}", dsu.get_class_num(qq.1 - 1, qq.2));
        }
    }
}

mod dsu {
    #[derive(Debug, Clone)]
    enum Node {
        Root(usize),
        Child(usize),
    }
    ///UnionFind
    #[derive(Clone, Debug)]
    pub struct Dsu {
        uf: Vec<Node>,
        data: Vec<Option<std::collections::BTreeMap<usize, usize>>>,
    }

    impl Dsu {
        pub fn new(c: Vec<usize>) -> Dsu {
            let b: Vec<_> = c
                .iter()
                .map(|x| {
                    Some({
                        let mut aaa = std::collections::BTreeMap::new();
                        aaa.insert(*x, 1);

                        aaa
                    })
                })
                .collect();

            Dsu {
                uf: vec![Node::Root(1); c.len()],
                data: b,
            }
        }

        pub fn root(&mut self, target: usize) -> usize {
            match self.uf[target] {
                Node::Root(_) => target,
                Node::Child(par) => {
                    let root = self.root(par);
                    self.uf[target] = Node::Child(self.root(par));
                    root
                }
            }
        }
        pub fn unite(&mut self, x: usize, y: usize) -> bool {
            let rx = self.root(x);
            let ry = self.root(y);
            if rx == ry {
                return false;
            }
            let size_x = self.size(x);
            let size_y = self.size(y);

            let (i, j) = if size_x > size_y { (rx, ry) } else { (ry, rx) };

            for (k, v) in self.data[j].take().unwrap().iter() {
                *self.data[i].as_mut().unwrap().entry(*k).or_insert(0) += v;
            }

            self.uf[i] = Node::Root(size_x + size_y);
            self.uf[j] = Node::Child(i);

            true
        }
        pub fn is_same(&mut self, x: usize, y: usize) -> bool {
            self.root(x) == self.root(y)
        }
        pub fn size(&mut self, x: usize) -> usize {
            let root = self.root(x);
            match self.uf[root] {
                Node::Root(size) => size,
                Node::Child(_) => 0,
            }
        }
        pub fn get_same_group(&mut self, x: usize) -> std::collections::HashSet<usize> {
            let root = self.root(x);
            let mut g = std::collections::HashSet::new();
            for i in 0..self.uf.len() {
                if root == self.root(i) {
                    g.insert(i);
                }
            }
            g
        }
        pub fn get_all_groups(
            &mut self,
        ) -> std::collections::HashMap<usize, std::collections::HashSet<usize>> {
            let mut map: std::collections::HashMap<usize, std::collections::HashSet<usize>> =
                std::collections::HashMap::new();
            for i in 0..self.uf.len() {
                let root = self.root(i);

                map.entry(root)
                    .or_insert_with(std::collections::HashSet::new)
                    .insert(i);
            }
            map
        }
        pub fn get_class_num(&mut self, x: usize, y: usize) -> usize {
            let root = self.root(x);
            *self.data[root].as_ref().unwrap().get(&y).unwrap_or(&0)
        }
    }
}
