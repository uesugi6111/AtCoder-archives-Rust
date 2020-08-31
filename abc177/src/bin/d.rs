use proconio::{fastout, input, marker::Usize1};
#[fastout]
fn main() {
    input!(n: usize, m: usize, ab: [[Usize1; 2]; m]);

    let mut unionf = UnionFind::new(n);
    for v in ab {
        unionf.unite(v[0], v[1]);
    }

    println!("{}", (0..n).map(|x| unionf.size(x)).max().unwrap());
}

#[derive(Debug, Clone)]
enum Node {
    Root(usize),
    Child(usize),
}
#[derive(Clone, Debug)]
pub struct UnionFind {
    uf: Vec<Node>,
}

impl UnionFind {
    pub fn new(n: usize) -> UnionFind {
        UnionFind {
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
    pub fn size(&mut self, x: usize) -> usize {
        let root = self.root(x);
        match self.uf[root] {
            Node::Root(size) => size,
            Node::Child(_) => 0,
        }
    }
}
