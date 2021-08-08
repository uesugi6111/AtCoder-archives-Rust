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
#[proconio::fastout]
fn main() {
    input!(n: usize, ab: [(i64, i64); n - 1]);

    let mut e = vec![vec![]; n];
    for i in 0..n - 1 {
        e[ab[i].0 as usize - 1].push(ab[i].1 as usize - 1);
        e[ab[i].1 as usize - 1].push(ab[i].0 as usize - 1);
    }

    let ans = euler_tour::euler_tour(&e, 0);

    for (i, e) in ans.0.iter().enumerate() {
        if i == ans.0.len() - 1 {
            println!("{}", e + 1);
        } else {
            print!("{} ", e + 1);
        }
    }
}

mod euler_tour {
    #[derive(Debug)]
    pub enum Vertex {
        In(usize),
        Out(usize),
    }
    impl Vertex {
        pub fn get_value(&self) -> usize {
            match self {
                Vertex::In(value) => *value,
                Vertex::Out(value) => *value,
            }
        }
    }
    use std::collections::VecDeque;

    use itertools::Itertools;

    pub fn euler_tour(e: &[Vec<usize>], root: usize) -> (Vec<usize>, Vec<usize>, Vec<usize>) {
        let mut stack = VecDeque::new();
        stack.push_back(Vertex::In(root));
        let mut tour = vec![];
        let mut first_look = vec![None; e.len()];
        let mut depth = 0;
        let mut depths = vec![0; e.len()];
        while let Some(vertex) = stack.pop_back() {
            if let Vertex::In(v) = vertex {
                for &to in e[v].iter().sorted_by_key(|&&x| x.wrapping_neg()) {
                    if first_look[to].is_some() {
                        continue;
                    }
                    stack.push_back(Vertex::Out(v));
                    stack.push_back(Vertex::In(to));
                }
                first_look[v] = Some(tour.len());
                depths[v] = depth;
                depth += 1;
            } else {
                depth -= 1;
            }
            tour.push(vertex.get_value());
        }

        (
            tour,
            first_look.iter().map(|x| x.unwrap()).collect(),
            depths,
        )
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_eiler_tour() {
            let e = vec![vec![5, 1], vec![4, 2], vec![3], vec![], vec![], vec![]];
            let (ans, _, _) = euler_tour(&e, 0);
            assert_eq!(&ans, &[0, 1, 2, 3, 2, 1, 4, 1, 0, 5, 0]);
        }
    }
}
