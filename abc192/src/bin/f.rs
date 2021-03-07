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
            let mut start=None;loop{match(self.buf[self.pos],start.is_some()){(b' ',true)|(b'\n', true)|(b'\r', true)=>break,(_, true)|(b' ', false)|(b'\n',false)|(b'\r', false)=>self.pos+=1,(_, false)=>start=Some(self.pos),}}let target=&self.buf[start.unwrap()..self.pos];
            unsafe { std::str::from_utf8_unchecked(target) }.parse().unwrap()
        }
    }
}
use std::io::Write;
fn main() {
    input!(n: usize, q: usize, a: [i64; n], lr: [(usize, usize); q]);
    let sp = sp::SparseTable::new(&a);

    let out = std::io::stdout();
    let mut out = std::io::BufWriter::new(out.lock());
    for (l, r) in lr {
        writeln!(&mut out, "{}", sp.query(l, r)).unwrap();
    }
}

mod sp {
    pub struct SparseTable {
        v: Vec<i64>,
        log_table: Vec<usize>,
        table: Vec<Vec<usize>>,
    }

    impl SparseTable {
        pub fn new(v: &[i64]) -> Self {
            let mut log_table = vec![0; v.len() + 1];
            for i in 2..=v.len() {
                log_table[i] = log_table[i >> 1] + 1;
            }
            let mut table: Vec<Vec<usize>> = (0..v.len())
                .map(|i| vec![i; log_table[v.len() - i] + 1])
                .collect();
            for k in 1..=log_table[v.len()] {
                for i in 0..=v.len() - (1 << k) {
                    let index_1 = table[i][k - 1];
                    let index_2 = table[i + (1 << (k - 1))][k - 1];
                    table[i][k] = if v[index_1] < v[index_2] {
                        index_1
                    } else {
                        index_2
                    };
                }
            }
            SparseTable {
                v: v.to_vec(),
                log_table,
                table,
            }
        }
        pub fn query(&self, l: usize, r: usize) -> i64 {
            if l == r {
                self.v[l]
            } else {
                let i = self.log_table[r - l];
                std::cmp::min(
                    self.v[self.table[l][i]],
                    self.v[self.table[r - (1 << i)][i]],
                )
            }
        }
    }

    #[test]
    fn test_sparse_table() {
        let a = SparseTable::new(&[2, 10, 1, 100]);
        for (l, r, ans) in [
            (0, 1, 2),
            (0, 2, 2),
            (0, 3, 1),
            (0, 4, 1),
            (1, 2, 10),
            (1, 3, 1),
            (1, 4, 1),
            (2, 3, 1),
            (2, 4, 1),
            (3, 4, 100),
        ]
        .iter()
        {
            assert_eq!(a.query(*l, *r), *ans);
        }
    }
}
