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
use std::io::Write;
fn main() {
    input!(n: usize, m: usize, ab: [(usize, usize); m]);
    let mut e = vec![vec![]; n];
    for &(v, u) in ab.iter() {
        e[v].push(u);
    }
    let a = scc::decompose(&e);
    let out = std::io::stdout();
    let mut out = std::io::BufWriter::new(out.lock());
    writeln!(&mut out, "{}", a.len()).ok();
    for v in a.iter() {
        write!(&mut out, "{}", v.len()).ok();

        for i in v.iter() {
            write!(&mut out, " {}", i).ok();
        }
        writeln!(&mut out).ok();
    }
}

mod scc {
    #[derive(Debug)]
    enum Vertex {
        In(usize),
        Out(usize),
    }

    pub fn decompose(e: &[Vec<usize>]) -> Vec<Vec<usize>> {
        let mut seen = vec![false; e.len()];

        let mut stack = vec![];
        let mut nodes = Vec::with_capacity(e.len());
        for i in 0..e.len() {
            if seen[i] {
                continue;
            }
            stack.push(Vertex::In(i));

            while let Some(vertex) = stack.pop() {
                if let Vertex::In(v) = vertex {
                    if seen[v] {
                        continue;
                    }
                    stack.push(Vertex::Out(v));
                    seen[v] = true;
                    for &to in e[v].iter() {
                        stack.push(Vertex::In(to));
                    }
                } else if let Vertex::Out(v) = vertex {
                    nodes.push(v);
                }
            }
        }
        let mut reverse_edge = vec![vec![]; e.len()];
        for i in 0..e.len() {
            for j in 0..e[i].len() {
                reverse_edge[e[i][j]].push(i);
            }
        }

        let mut components = vec![];
        let mut back_stack = vec![];
        let mut back_seen = vec![false; e.len()];
        while let Some(v) = nodes.pop() {
            if back_seen[v] {
                continue;
            }
            let mut scc = vec![];
            back_stack.push(v);
            back_seen[v] = true;

            while let Some(v) = back_stack.pop() {
                for &to in reverse_edge[v].iter() {
                    if back_seen[to] {
                        continue;
                    }
                    back_stack.push(to);
                    back_seen[to] = true;
                }

                scc.push(v);
            }
            components.push(scc);
        }
        components
    }
}
