#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
fn main() {
    input!(n: usize, ab: [(usize, usize); n]);

    let mut dsu = dsu::Dsu::new(400_001);

    for (a, b) in ab {
        if dsu.is_same(a, b) {
            dsu.unite(a, 0);
        } else {
            dsu.unite(a, b);
        }
    }

    let ans: usize = (0..400_001)
        .map(|x| if dsu.is_root(x) { dsu.size(x) - 1 } else { 0 })
        .sum();

    println!("{}", ans);
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
    }

    impl Dsu {
        pub fn new(n: usize) -> Dsu {
            Dsu {
                uf: vec![Node::Root(1); n],
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
        pub fn is_root(&self, target: usize) -> bool {
            match self.uf[target] {
                Node::Root(_) => true,
                Node::Child(_) => false,
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
            self.uf[i] = Node::Root(size_x + size_y);
            self.uf[j] = Node::Child(i);

            true
        }
        pub fn is_same(&mut self, x: usize, y: usize) -> bool {
            self.root(x) == self.root(y)
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
                if i == root {
                    continue;
                }

                map.entry(root)
                    .or_insert_with(std::collections::HashSet::new)
                    .insert(i);
            }
            map
        }
        pub fn size(&mut self, x: usize) -> usize {
            let root = self.root(x);
            match self.uf[root] {
                Node::Root(size) => size,
                Node::Child(_) => 0,
            }
        }
    }
}
