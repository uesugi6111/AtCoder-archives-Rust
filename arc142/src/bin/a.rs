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
    input!(n: i64, k: i64);

    let l = k.to_string().len();

    let p = make_permutation(l);
    let kk = k.to_string().chars().collect::<Vec<_>>();

    let mut ans = vec![];

    for i in p.iter() {
        let mut s = String::new();
        for &j in i.iter() {
            s = format!("{}{}", s, kk[j]);
        }
        if s.parse::<i64>().unwrap_or(0).to_string().len() == l {
            for j in 0..12 {
                if n < s.parse::<i64>().unwrap() * 10i64.pow(j) {
                    break;
                }
                ans.push(s.parse::<i64>().unwrap() * 10i64.pow(j));
            }
        }
    }

    println!("{}", ans.iter().filter(|&x| *x <= n).count());
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
