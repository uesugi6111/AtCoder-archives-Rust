use std::io::{self, BufReader};

use itertools::Itertools;
use proconio::{source::line::LineSource, *};
fn main() {
    let mut source = LineSource::new(BufReader::new(io::stdin()));

    input!(from &mut source, n: usize,w:usize,k:usize,c:usize,ab:[(usize,usize);w],mut cd:[(usize,usize);k]);
    cd.push(ab[0]);
    let mut dsu = dsu::DisjointSetUnion::new(cd.len());
    let mut v = vec![];
    for i in 0..cd.len() - 1 {
        let mut index = std::usize::MAX;

        let aa = (0..cd.len())
            .map(|x| {
                (
                    x,
                    (cd[i].0 as i64 - cd[x].0 as i64).abs()
                        + (cd[i].1 as i64 - cd[x].1 as i64).abs(),
                )
            })
            .sorted_by_key(|x| x.1)
            .collect::<Vec<_>>();
        for (j, _) in aa {
            if i == j {
                continue;
            }

            if dsu.unite(i, j) {
                index = j;
                break;
            }
        }
        if index == std::usize::MAX {
            continue;
        }
        v.push((i, index));
    }
    let mut map = std::collections::BTreeMap::new();
    if !cfg!(debug_assertions) {
        for i in (1..).map(|x| x * 17).take_while(|&x| x < 175.min(n)) {
            for j in (1..).map(|x| x * 17).take_while(|&x| x < 175.min(n)) {
                println!("{} {} {}", i, j, 350);
                input!(from &mut source,r: i32);
                if r == 1 {
                    map.entry(i).or_insert_with(Vec::new).push(j);
                }
            }
        }
        let mut l = v.len() - 1;
        loop {
            'outer: for &x in &map
                .range(cd[v[l].0].0.min(cd[v[l].1].0)..=cd[v[l].0].0.max(cd[v[l].1].0))
                .next()
            {
                for &i in x.1 {
                    if cd[v[l].0].1.min(cd[v[l].1].1) <= i && i <= cd[v[l].0].1.max(cd[v[l].1].1) {
                        cd.push((*x.0, i));
                        let buff = v[l];
                        v[l] = (buff.0, cd.len() - 1);
                        v.push((cd.len() - 1, buff.1));
                        break 'outer;
                    }
                }
            }
            if l == 0 {
                break;
            }
            l -= 1;
        }
    }

    let mut nn = vec![vec![false; n]; n];

    for &(i, j) in &v {
        let (c, d) = if cd[i].0 < cd[j].0 {
            (cd[i], cd[j])
        } else {
            (cd[j], cd[i])
        };
        for index in c.0..=d.0 {
            nn[index][c.1] = true;
        }
        for index in c.1.min(d.1)..=c.1.max(d.1) {
            nn[d.0][index] = true;
        }
    }

    for (i, j) in &map {
        for jj in j {
            nn[*i][*jj] = false;
        }
    }
    if cfg!(debug_assertions) {
        for i in 0..n {
            println!(
                "{:?}",
                &nn[i]
                    .iter()
                    .map(|&x| if x { '.' } else { '#' })
                    .collect::<String>()
            );
        }
        return;
    }

    for i in 0..n {
        for j in 0..n {
            if !nn[i][j] {
                continue;
            }
            loop {
                println!("{} {} {}", i, j, 800 / (128 / c));
                input!(from &mut source,r: i32);
                if r == 2 {
                    return;
                } else if r == 1 {
                    break;
                }
            }
        }
    }
}

mod dsu {
    //! Union find
    use std::collections::{HashMap, HashSet};
    #[derive(Copy, Clone, Debug)]
    enum Node {
        Root(usize),
        Child(usize),
    }
    ///UnionFind
    #[derive(Clone, Debug)]
    pub struct DisjointSetUnion {
        nodes: Vec<Node>,
    }

    impl DisjointSetUnion {
        pub fn new(n: usize) -> DisjointSetUnion {
            DisjointSetUnion {
                nodes: vec![Node::Root(1); n],
            }
        }

        pub fn find_root(&mut self, target: usize) -> usize {
            match unsafe { *self.nodes.get_unchecked(target) } {
                Node::Root(_) => target,
                Node::Child(parent) => {
                    let parent_index = self.find_root(parent);
                    self.nodes[target] = Node::Child(parent_index);
                    parent_index
                }
            }
        }
        pub fn unite(&mut self, x: usize, y: usize) -> bool {
            let rx = self.find_root(x);
            let ry = self.find_root(y);
            if rx == ry {
                return false;
            }
            let size_x = self.size(x);
            let size_y = self.size(y);

            let (i, j) = if size_x > size_y { (rx, ry) } else { (ry, rx) };
            self.nodes[i] = Node::Root(size_x + size_y);
            self.nodes[j] = Node::Child(i);

            true
        }
        pub fn is_same(&mut self, x: usize, y: usize) -> bool {
            self.find_root(x) == self.find_root(y)
        }
        pub fn size(&mut self, x: usize) -> usize {
            let root = self.find_root(x);
            match self.nodes[root] {
                Node::Root(size) => size,
                Node::Child(_) => 0,
            }
        }
        pub fn get_same_group(&mut self, x: usize) -> HashSet<usize> {
            let root = self.find_root(x);
            let mut g = HashSet::new();
            for i in 0..self.nodes.len() {
                if root == self.find_root(i) {
                    g.insert(i);
                }
            }
            g
        }
        pub fn get_all_groups(&mut self) -> HashMap<usize, HashSet<usize>> {
            let mut map = HashMap::new();
            for i in 0..self.nodes.len() {
                map.entry(self.find_root(i))
                    .or_insert_with(HashSet::new)
                    .insert(i);
            }
            map
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_dsu() {
            let mut d = DisjointSetUnion::new(4);
            d.unite(0, 1);
            assert!(d.is_same(0, 1));
            d.unite(1, 2);
            assert!(d.is_same(0, 2));
            assert_eq!(d.size(0), 3);
            assert!(!d.is_same(0, 3));

            // assert_eq!(d.get_all_groups(), vec![vec![0, 1, 2], vec![3]]);
        }
    }
}
