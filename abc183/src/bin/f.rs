#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
fn main() {
    input!(
        n: usize,
        q: usize,
        c: [usize; n],
        query: [(usize, usize, usize); q]
    );

    let mut dsu = dsu::Dsu::new(c);

    for qq in query {
        if qq.0 == 1 {
            dsu.unite(qq.1, qq.2);
        } else {
            println!("{}", dsu.get_class_num(qq.1, qq.2));
        }
    }
}

mod dsu {
    #[derive(Debug, Clone)]
    enum Node {
        Root(usize, std::collections::HashMap<usize, usize>),
        Child(usize),
    }
    ///UnionFind
    #[derive(Clone, Debug)]
    pub struct Dsu {
        uf: Vec<Node>,
    }

    impl Dsu {
        pub fn new(c: Vec<usize>) -> Dsu {
            let mut a = vec![Node::Root(1, std::collections::HashMap::new()); 1];
            let mut b: Vec<_> = c
                .iter()
                .map(|x| -> Node {
                    let mut aaa = std::collections::HashMap::new();
                    aaa.insert(*x, 1);

                    Node::Root(1, aaa)
                })
                .collect();

            a.append(&mut b);

            Dsu { uf: a }
        }

        pub fn root(&mut self, target: usize) -> usize {
            match self.uf[target] {
                Node::Root(_, _) => target,
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

            let map = if let Node::Root(_, map) = &self.uf[i] {
                let mut m = map.clone();
                if let Node::Root(_, a) = &self.uf[j] {
                    for v in a.keys() {
                        *m.entry(*v).or_insert(0) += *(a.get(v).unwrap_or(&0));
                    }
                }
                m
            } else {
                std::collections::HashMap::new()
            };

            self.uf[i] = Node::Root(size_x + size_y, map);
            self.uf[j] = Node::Child(i);

            true
        }
        pub fn is_same(&mut self, x: usize, y: usize) -> bool {
            self.root(x) == self.root(y)
        }
        pub fn size(&mut self, x: usize) -> usize {
            let root = self.root(x);
            match self.uf[root] {
                Node::Root(size, _) => size,
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
            match &self.uf[root] {
                Node::Root(_, a) => *a.get(&y).unwrap_or(&0),
                Node::Child(_) => 0,
            }
        }
    }
}
