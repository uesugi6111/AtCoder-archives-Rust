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

pub const DI: [(char, (i64, i64)); 4] =
    [('L', (-1, 0)), ('U', (0, -1)), ('R', (1, 0)), ('D', (0, 1))];
use std::time::Instant;
#[proconio::fastout]
fn main() {
    let since = Instant::now();

    let inp = parse_input();
    let mut zero = None;
    for i in 0..inp.n {
        for j in 0..inp.n {
            if inp.tiles[i][j] == 0 {
                zero = Some((j as i64, i as i64));
            }
        }
    }

    let mut time = 0;
    let mut all_iter = 0;
    let mut count = 0;

    let mut max = (0, vec![], zero.unwrap());

    //let mut ans = vec![];
    let mut xs = xs::XorShift::new();
    const it: [u128; 4] = [50, 1000, 2000, 2960];
    const CCC: [usize; 4] = [20, 20, 20, 20];
    for i in 0..it.len() {
        let mut max2 = (max.0, vec![], max.2);
        while time < it[i] {
            all_iter += 1;
            if (all_iter & ((1 << 4) - 1)) == 0 {
                time = Instant::now().duration_since(since).as_millis();
            }
            let mut a = vec![];
            let (mut x, mut y) = zero.unwrap();
            for i in 0..CCC[i as usize] {
                let mut buff = xs.next().unwrap() as usize % 4;
                while !(0..inp.n as i64).contains(&(x + (DI[buff].1).0))
                    || !(0..inp.n as i64).contains(&(y + (DI[buff].1).1))
                {
                    buff = xs.next().unwrap() as usize % 4;
                }
                a.push(DI[buff].0);
                x += (DI[buff].1).0;
                y += (DI[buff].1).1;
            }
            let (score, _, _) = if i == 0 {
                compute_score2(
                    &inp,
                    &max.1
                        .iter()
                        .copied()
                        .chain(a.iter().copied())
                        .collect::<Vec<_>>(),
                )
            } else {
                compute_score(
                    &inp,
                    &max.1
                        .iter()
                        .copied()
                        .chain(a.iter().copied())
                        .collect::<Vec<_>>(),
                )
            };

            if max2.0 < score {
                max2 = (score, a, (x, y));
            }
        }
        if max.1.len() + max2.1.len() > inp.T {
            break;
        }
        max = (
            max2.0,
            max.1
                .iter()
                .copied()
                .chain(max2.1.iter().copied())
                .collect::<Vec<_>>(),
            max2.2,
        );
        eprintln!(r#"{}"#, &max.0);
    }

    eprintln!(r#"{} / {}"#, &count, &all_iter);

    println!("{}", max.1.iter().collect::<String>());
}

pub trait SetMinMax {
    fn setmin(&mut self, v: Self) -> bool;
    fn setmax(&mut self, v: Self) -> bool;
}
impl<T> SetMinMax for T
where
    T: PartialOrd,
{
    fn setmin(&mut self, v: T) -> bool {
        *self > v && {
            *self = v;
            true
        }
    }
    fn setmax(&mut self, v: T) -> bool {
        *self < v && {
            *self = v;
            true
        }
    }
}

#[macro_export]
macro_rules! mat {
	($($e:expr),*) => { Vec::from(vec![$($e),*]) };
	($($e:expr,)*) => { Vec::from(vec![$($e),*]) };
	($e:expr; $d:expr) => { Vec::from(vec![$e; $d]) };
	($e:expr; $d:expr $(; $ds:expr)+) => { Vec::from(vec![mat![$e $(; $ds)*]; $d]) };
}

use std::cell::Cell;

#[derive(Clone, Debug)]
pub struct UnionFind {
    /// size / parent
    ps: Vec<Cell<usize>>,
    pub is_root: Vec<bool>,
}

impl UnionFind {
    pub fn new(n: usize) -> UnionFind {
        UnionFind {
            ps: vec![Cell::new(1); n],
            is_root: vec![true; n],
        }
    }
    pub fn find(&self, x: usize) -> usize {
        if self.is_root[x] {
            x
        } else {
            let p = self.find(self.ps[x].get());
            self.ps[x].set(p);
            p
        }
    }
    pub fn unite(&mut self, x: usize, y: usize) {
        let mut x = self.find(x);
        let mut y = self.find(y);
        if x == y {
            return;
        }
        if self.ps[x].get() < self.ps[y].get() {
            ::std::mem::swap(&mut x, &mut y);
        }
        *self.ps[x].get_mut() += self.ps[y].get();
        self.ps[y].set(x);
        self.is_root[y] = false;
    }
    pub fn same(&self, x: usize, y: usize) -> bool {
        self.find(x) == self.find(y)
    }
    pub fn size(&self, x: usize) -> usize {
        self.ps[self.find(x)].get()
    }
}

pub type Output = Vec<char>;

pub const DIJ: [(usize, usize); 4] = [(0, !0), (!0, 0), (0, 1), (1, 0)];
pub const DIR: [char; 4] = ['L', 'U', 'R', 'D'];

#[derive(Clone, Debug)]
pub struct Input {
    pub n: usize,
    pub T: usize,
    pub tiles: Vec<Vec<usize>>,
}

impl std::fmt::Display for Input {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{} {}", self.n, self.T)?;
        for i in 0..self.n {
            for j in 0..self.n {
                write!(f, "{:0x}", self.tiles[i][j])?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

pub fn parse_input() -> Input {
    input!(n: usize, T: usize, tiles: [Chars; n]);
    let tiles = tiles
        .iter()
        .map(|ts| {
            ts.iter()
                .map(|&c| usize::from_str_radix(&c.to_string(), 16).unwrap())
                .collect()
        })
        .collect();
    Input { n, T, tiles }
}

pub fn parse_output(_input: &Input, f: &str) -> Result<Output, String> {
    Ok(f.trim().chars().collect())
}

pub struct Sim {
    n: usize,
    T: usize,
    from: Vec<Vec<(usize, usize)>>,
    turn: usize,
    i: usize,
    j: usize,
}

impl Sim {
    pub fn new(input: &Input) -> Self {
        let mut i = !0;
        let mut j = !0;
        let mut from = mat![(0, 0); input.n; input.n];
        for x in 0..input.n {
            for y in 0..input.n {
                if input.tiles[x][y] == 0 {
                    i = x;
                    j = y;
                }
                from[x][y] = (x, y);
            }
        }
        Sim {
            n: input.n,
            T: input.T,
            from,
            turn: 0,
            i,
            j,
        }
    }
    pub fn apply(&mut self, c: char) -> Result<(), String> {
        if let Some(d) = DIR.iter().position(|&d| d == c) {
            let i2 = self.i + DIJ[d].0;
            let j2 = self.j + DIJ[d].1;
            if i2 >= self.n || j2 >= self.n {
                Err(format!("illegal move: {} (turn {})", c, self.turn))
            } else {
                let f1 = self.from[self.i][self.j];
                let f2 = self.from[i2][j2];
                self.from[i2][j2] = f1;
                self.from[self.i][self.j] = f2;
                self.i = i2;
                self.j = j2;
                self.turn += 1;
                Ok(())
            }
        } else {
            Err(format!("illegal move: {} (turn {})", c, self.turn))
        }
    }
    pub fn compute_score(&self, input: &Input) -> (i64, i64, String, Vec<Vec<bool>>) {
        let mut uf = UnionFind::new(self.n * self.n);
        let mut tree = vec![true; self.n * self.n];
        let mut tiles = mat![0; self.n; self.n];
        for i in 0..self.n {
            for j in 0..self.n {
                tiles[i][j] = input.tiles[self.from[i][j].0][self.from[i][j].1];
            }
        }
        for i in 0..self.n {
            for j in 0..self.n {
                if i + 1 < self.n && tiles[i][j] & 8 != 0 && tiles[i + 1][j] & 2 != 0 {
                    let a = uf.find(i * self.n + j);
                    let b = uf.find((i + 1) * self.n + j);
                    if a == b {
                        tree[a] = false;
                    } else {
                        let t = tree[a] && tree[b];
                        uf.unite(a, b);
                        tree[uf.find(a)] = t;
                    }
                }
                if j + 1 < self.n && tiles[i][j] & 4 != 0 && tiles[i][j + 1] & 1 != 0 {
                    let a = uf.find(i * self.n + j);
                    let b = uf.find(i * self.n + j + 1);
                    if a == b {
                        tree[a] = false;
                    } else {
                        let t = tree[a] && tree[b];
                        uf.unite(a, b);
                        tree[uf.find(a)] = t;
                    }
                }
            }
        }
        let mut max_tree = !0;
        for i in 0..self.n {
            for j in 0..self.n {
                if tiles[i][j] != 0 && tree[uf.find(i * self.n + j)] {
                    if max_tree == !0 || uf.size(max_tree) < uf.size(i * self.n + j) {
                        max_tree = i * self.n + j;
                    }
                }
            }
        }
        let mut bs = mat![false; self.n; self.n];
        if max_tree != !0 {
            for i in 0..self.n {
                for j in 0..self.n {
                    bs[i][j] = uf.same(max_tree, i * self.n + j);
                }
            }
        }
        if self.turn > self.T {
            return (0, 0, format!("too many moves"), bs);
        }
        let size = if max_tree == !0 { 0 } else { uf.size(max_tree) };
        let score = if size == self.n * self.n - 1 {
            (500000.0 * (1.0 + (self.T - self.turn) as f64 / self.T as f64)).round()
        } else {
            (500000.0 * size as f64 / (self.n * self.n - 1) as f64).round()
        } as i64;

        let mut minus = 0;
        for i in 0..self.n {
            if tiles[0][i] & 2 == 2 {
                minus += 1;
            }
            if tiles[self.n - 1][i] & 8 == 8 {
                minus += 1;
            }
            if tiles[i][0] & 1 == 1 {
                minus += 1;
            }
            if tiles[i][self.n - 1] & 4 == 4 {
                minus += 1;
            }
        }
        (score, minus, String::new(), bs)
    }
}

pub fn compute_score(
    input: &Input,
    out: &[char],
) -> (i64, String, (Vec<Vec<(usize, usize)>>, Vec<Vec<bool>>)) {
    let mut sim = Sim::new(input);
    for &c in out {
        if let Err(err) = sim.apply(c) {
            return (0, err, (sim.from.clone(), sim.compute_score(input).3));
        }
    }
    let (score, m, err, tree) = sim.compute_score(input);
    (score - m, err, (sim.from.clone(), tree))
}
pub fn compute_score2(
    input: &Input,
    out: &[char],
) -> (i64, String, (Vec<Vec<(usize, usize)>>, Vec<Vec<bool>>)) {
    let mut sim = Sim::new(input);
    for &c in out {
        if let Err(err) = sim.apply(c) {
            return (0, err, (sim.from.clone(), sim.compute_score(input).3));
        }
    }
    let (score, m, err, tree) = sim.compute_score(input);
    (100 - m, err, (sim.from.clone(), tree))
}

mod xs {
    //! Xorshift random number generator
    use std::time::SystemTime;

    #[derive(Debug, Default, Clone, Copy)]
    pub struct XorShift {
        seed: u64,
    }

    impl XorShift {
        pub fn new() -> Self {
            Self {
                seed: SystemTime::now()
                    .duration_since(SystemTime::UNIX_EPOCH)
                    .unwrap()
                    .as_secs(),
            }
        }
        pub fn from_seed(seed: u64) -> Self {
            Self { seed }
        }
    }

    impl Iterator for XorShift {
        type Item = u64;

        fn next(&mut self) -> Option<Self::Item> {
            self.seed ^= self.seed << 13;
            self.seed ^= self.seed >> 7;
            self.seed ^= self.seed << 17;
            Some(self.seed)
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use std::collections::HashSet;
        #[test]
        fn test_xorshift() {
            let mut set = HashSet::new();
            let xorshift = XorShift::new();

            for v in xorshift.take(100_000) {
                assert!(!set.contains(&v));
                set.insert(v);
            }
        }
    }
}
