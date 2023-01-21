#[rustfmt::skip]
mod io_pro {
    #[macro_export] macro_rules! input{(sc=$sc:expr,$($r:tt)*)=>{input_inner!{$sc,$($r)*}};($($r:tt)*)=>{let mut sc=io_pro::Scanner::new(std::io::BufReader::new(std::io::stdin().lock()));input_inner!{sc,$($r)*}};}
    #[macro_export] macro_rules! input_inner{($sc:expr)=>{};($sc:expr,)=>{};($sc:expr,$var:ident:$t:tt$($r:tt)*)=>{let $var=read_value!($sc,$t);input_inner!{$sc $($r)*}};($sc:expr,mut $var:ident:$t:tt$($r:tt)*)=>{let mut $var=read_value!($sc,$t);input_inner!{$sc $($r)*}};}
    #[macro_export] macro_rules! read_value{($sc:expr,($($t:tt),*))=>{($(read_value!($sc,$t)),*)};($sc:expr,[$t:tt;$len:expr])=>{(0..$len).map(|_|read_value!($sc,$t)).collect::<Vec<_>>()};($sc:expr,Chars)=>{read_value!($sc,String).chars().collect::<Vec<char>>()};($sc:expr,Usize1)=>{read_value!($sc,usize)-1};($sc:expr,$t:ty)=>{$sc.next::<$t>()};}
    pub struct Scanner{s:Box<str>,input:std::str::SplitAsciiWhitespace<'static>,}
    impl Scanner{
        pub fn new<R:std::io::Read>(mut reader:R)->Self{let mut sc=Scanner{s:{let mut s=String::new();reader.read_to_string(&mut s).unwrap();s.into_boxed_str()},input:"".split_ascii_whitespace(),};let s:&'static str=unsafe{std::mem::transmute(&*sc.s)};sc.input=s.split_ascii_whitespace();sc}
        #[inline]pub fn next<T:std::str::FromStr>(&mut self)->T where T::Err:std::fmt::Debug,{self.input.next().unwrap().parse::<T>().expect("Parse error")}
    }
}
#[proconio::fastout]
fn main() {
    input!(
        n: usize,
        a: [i64; n],
        s: [Chars; n],
        q: usize,
        uv: [(usize, usize); q]
    );

    let mut v = vec![vec![None; n]; n];

    for i in 0..n {
        for j in 0..n {
            if s[i][j] == 'Y' {
                v[i][j] = Some(1);
            }
        }
    }

    let mut sss = floyd_warshall(&v);

    for i in 0..q {
        if let Some(buff) = sss[uv[i].0 - 1][uv[i].1 - 1].as_ref() {
            let mut sum = 0;
            for &k in &buff.1 {
                sum += a[k];
            }
            println!("{} {}", buff.0, sum);
        } else {
            println!("Impossible");
        }
    }
}

pub fn floyd_warshall(
    matrix: &[Vec<Option<i64>>],
) -> Vec<Vec<Option<(i64, std::collections::HashSet<usize>)>>> {
    let mut m = vec![vec![None; matrix[0].len()]; matrix.len()];

    for i in 0..matrix.len() {
        for j in 0..matrix.len() {
            if matrix[i][j].is_none() {
                continue;
            }
            let mut set = std::collections::HashSet::new();
            set.insert(i);
            set.insert(j);

            m[i][j] = Some((matrix[i][j].unwrap(), set));
        }
    }
    let n = m.len();
    (0..n).for_each(|i| {
        (0..n).for_each(|j| {
            (0..n).for_each(|k| {
                m[j][k] = if m[j][k].is_none() && m[j][i].is_none() && m[i][k].is_none() {
                    None
                } else if m[j][i].is_none() || m[i][k].is_none() {
                    m[j][k].clone()
                } else if m[j][k].is_none() {
                    Some((
                        m[j][i].as_ref().unwrap().0 + m[i][k].as_ref().unwrap().0,
                        m[j][i]
                            .as_ref()
                            .unwrap()
                            .1
                            .union(&m[i][k].as_ref().unwrap().1)
                            .copied()
                            .collect::<std::collections::HashSet<_>>(),
                    ))
                } else {
                    if m[j][k].as_ref().unwrap().0
                        < m[j][i].as_ref().unwrap().0 + m[i][k].as_ref().unwrap().0
                    {
                        m[j][k].clone()
                    } else {
                        Some((
                            m[j][i].as_ref().unwrap().0 + m[i][k].as_ref().unwrap().0,
                            m[j][i]
                                .as_ref()
                                .unwrap()
                                .1
                                .union(&m[i][k].as_ref().unwrap().1)
                                .copied()
                                .collect::<std::collections::HashSet<_>>(),
                        ))
                    }

                    // Some(std::cmp::min(
                    //     m[j][k].unwrap(),
                    //     m[j][i].unwrap() + m[i][k].unwrap(),
                    // ))
                };
            })
        })
    });
    m
}
