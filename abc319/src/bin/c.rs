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
    input!(c: [[i64; 3]; 3]);

    let p = make_permutation(9);
    let mut v = vec![];
    if c[0][0] == c[0][1] {
        v.push(((0usize, 1usize), 2usize));
    }
    if c[0][0] == c[0][2] {
        v.push(((0, 2), 1));
    }
    if c[0][1] == c[0][2] {
        v.push(((1, 2), 0));
    }

    if c[1][0] == c[1][1] {
        v.push(((3, 4), 5));
    }
    if c[1][0] == c[1][2] {
        v.push(((3, 5), 4));
    }
    if c[1][1] == c[1][2] {
        v.push(((4, 5), 3));
    }

    if c[2][0] == c[2][1] {
        v.push(((6, 7), 8));
    }
    if c[2][0] == c[2][2] {
        v.push(((6, 8), 7));
    }
    if c[2][1] == c[2][2] {
        v.push(((7, 8), 6));
    }

    if c[0][0] == c[1][0] {
        v.push(((0, 3), 6));
    }
    if c[0][0] == c[2][0] {
        v.push(((0, 6), 3));
    }
    if c[1][0] == c[2][0] {
        v.push(((3, 6), 0));
    }

    if c[0][1] == c[1][1] {
        v.push(((1, 4), 7));
    }
    if c[0][1] == c[2][1] {
        v.push(((1, 7), 4));
    }
    if c[1][1] == c[2][1] {
        v.push(((4, 7), 1));
    }

    if c[0][2] == c[1][2] {
        v.push(((2, 5), 8));
    }
    if c[0][2] == c[2][2] {
        v.push(((2, 8), 5));
    }
    if c[1][2] == c[2][2] {
        v.push(((5, 8), 2));
    }

    if c[0][0] == c[1][1] {
        v.push(((0, 4), 8));
    }
    if c[0][0] == c[2][2] {
        v.push(((0, 8), 4));
    }
    if c[1][1] == c[2][2] {
        v.push(((4, 8), 0));
    }

    if c[0][2] == c[1][1] {
        v.push(((2, 4), 6));
    }
    if c[0][2] == c[2][0] {
        v.push(((2, 6), 4));
    }
    if c[1][1] == c[2][0] {
        v.push(((4, 6), 2));
    }
    let mut count = 0;
    for vv in &p {
        let mut set = std::collections::HashSet::new();

        'aaa: for &i in vv {
            set.insert(i);
            for &j in &v {
                if i == j.1 {
                    if set.contains(&j.0 .0) && set.contains(&j.0 .1) {
                        count += 1;
                        break 'aaa;
                    }
                }
            }
        }
    }

    println!("{}", (p.len() - count) as f64 / p.len() as f64);
}

pub fn make_permutation(n: usize) -> Vec<Vec<usize>> {
    let factorial = (1..=n).product();
    let mut vvec: Vec<Vec<usize>> = vec![Vec::new(); factorial];
    let nums: Vec<usize> = (0..n).collect();
    let indexes: Vec<usize> = (0..factorial).collect();
    push_recusive(nums, indexes, &mut vvec);
    vvec
}

fn push_recusive<T: Clone>(
    nums: Vec<T>,
    indexes: Vec<usize>,
    vvec: &mut Vec<Vec<T>>,
) -> &mut Vec<Vec<T>> {
    if nums.is_empty() {
        return vvec;
    }
    let block_size = (1..nums.len()).product();
    for (block_index, num) in nums.iter().enumerate() {
        for inner_index in 0..block_size {
            let index = indexes[block_size * block_index + inner_index];
            vvec[index].push(num.clone());
        }
        let new_nums = {
            let mut tmp = nums.clone();
            tmp.remove(block_index);
            tmp
        };
        let new_indexes: Vec<usize> = {
            let slice = &indexes[(block_size * block_index)..(block_size * (block_index + 1))];
            slice.to_vec()
        };
        push_recusive(new_nums, new_indexes, vvec);
    }
    vvec
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_prm() {
        let vv = make_permutation(4);
        assert_eq!(0, vv[0][0]);
    }
}
