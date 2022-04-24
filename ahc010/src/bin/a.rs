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
    let since = Instant::now();
    let duration_inv = 1.0 / 1.5;
    let mut time = (std::time::Instant::now() - since).as_secs_f64() * duration_inv;
    eprintln!(r#"{}"#, &time);

    input!(t: [Chars; 30]);
    let tiles = t
        .iter()
        .map(|ts| ts.iter().map(|&c| (c as u8 - b'0') as usize).collect())
        .collect::<Vec<Vec<usize>>>();
    let mut seed = (0..900).map(|_| 0).collect::<Vec<_>>();
    let mut xorshift = xorshift::XorShift::new();
    time = (std::time::Instant::now() - since).as_secs_f64() * duration_inv;
    eprintln!(r#"{}"#, &time);
    let mut max = 0;
    // while time < 1.0 {
    //     let mut all_iter = 0;
    //     all_iter += 1;
    //     if (all_iter & ((1 << 2) - 1)) == 0 {
    //         time = (std::time::Instant::now() - since).as_secs_f64() * duration_inv;
    //         dbg!(&time);
    //     }
    for _ in 0..10000 {
        let buff = (0..900)
            .map(|_| (xorshift.next().unwrap() as u64 % 3) as i32)
            .collect::<Vec<_>>();
        let s = compute_score(&tiles, &buff);
        if max < s {
            max = s;
            seed = buff;
        }
    }
    eprintln!(r#"{}"#, &max);
    let output = seed;
    println!(
        "{}",
        output
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join("")
    );
}

use rand::prelude::*;
use std::{f64::consts::PI, time::Instant};

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

pub type Output = Vec<i32>;

pub const N: usize = 30;
const DIJ: [(usize, usize); 4] = [(0, !0), (!0, 0), (0, 1), (1, 0)];

#[derive(Clone, Debug)]
pub struct Input {
    tiles: Vec<Vec<usize>>,
}

impl std::fmt::Display for Input {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..N {
            for j in 0..N {
                write!(f, "{}", self.tiles[i][j])?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

pub fn parse_output(_input: &Input, f: &str) -> Result<Vec<Output>, String> {
    let mut out = vec![];
    for line in f.lines() {
        let line = line.trim();
        let mut tmp = vec![];
        for c in line.chars() {
            if c < '0' || '3' < c {
                return Err(format!("Illegal output: {}", c));
            }
            tmp.push((c as u8 - b'0') as i32);
        }
        if tmp.len() != N * N {
            return Err(format!("Illegal output length: {}", line.len()));
        }
        out.push(tmp);
    }
    if out.len() == 0 {
        return Err(format!("empty output"));
    }
    Ok(out)
}

const ROTATE: [usize; 8] = [1, 2, 3, 0, 5, 4, 7, 6];

const TO: [[usize; 4]; 8] = [
    [1, 0, !0, !0],
    [3, !0, !0, 0],
    [!0, !0, 3, 2],
    [!0, 2, 1, !0],
    [1, 0, 3, 2],
    [3, 2, 1, 0],
    [2, !0, 0, !0],
    [!0, 3, !0, 1],
];

pub fn compute_score(input: &[Vec<usize>], out: &Output) -> i64 {
    let mut tiles = input.iter().cloned().collect::<Vec<_>>();
    for i in 0..N {
        for j in 0..N {
            for _ in 0..out[i * N + j] {
                tiles[i][j] = ROTATE[tiles[i][j]];
            }
        }
    }
    let mut ls = vec![];
    let mut used = mat![false; N; N; 4];
    let mut cycle = mat![(0, 0); N; N; 4];
    for i in 0..N {
        for j in 0..N {
            for d in 0..4 {
                if TO[tiles[i][j]][d] != !0 && !used[i][j][d] {
                    let mut i2 = i;
                    let mut j2 = j;
                    let mut d2 = d;
                    let mut length = 0;
                    let mut tmp = vec![];
                    while !used[i2][j2][d2] {
                        if TO[tiles[i2][j2]][d2] == !0 {
                            break;
                        }
                        length += 1;
                        used[i2][j2][d2] = true;
                        tmp.push((i2, j2, d2));
                        d2 = TO[tiles[i2][j2]][d2];
                        used[i2][j2][d2] = true;
                        tmp.push((i2, j2, d2));
                        i2 += DIJ[d2].0;
                        j2 += DIJ[d2].1;
                        if i2 >= N || j2 >= N {
                            break;
                        }
                        d2 = (d2 + 2) % 4;
                    }
                    if (i, j, d) == (i2, j2, d2) {
                        ls.push((length, tmp.clone()));
                        for (i, j, d) in tmp {
                            cycle[i][j][d].0 = length;
                        }
                    }
                }
            }
        }
    }
    let score = if ls.len() <= 1 {
        0
    } else {
        ls.sort();
        for &(i, j, d) in &ls[ls.len() - 1].1 {
            cycle[i][j][d].1 = 1;
        }
        for &(i, j, d) in &ls[ls.len() - 2].1 {
            cycle[i][j][d].1 = 2;
        }
        ls[ls.len() - 1].0 * ls[ls.len() - 2].0
    };
    score as i64
}

#[test]
fn a() {
    println!("{}", !0);
}

mod xorshift {
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
        #[test]
        fn test_xorshift2() {
            let mut set = HashSet::new();
            let xorshift = XorShift::new();

            for v in xorshift.take(100) {
                assert!(!set.contains(&v));
                set.insert(v);
            }

            dbg!("{}", set);
        }
    }
}
