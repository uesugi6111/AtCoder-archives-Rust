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
    input!(
        n: usize,
        m: usize,
        ab: [(usize, usize); m],
        cd: [(usize, usize); m]
    );

    let mut ab_map = std::collections::HashMap::new();

    let mut cd_map = std::collections::HashMap::new();

    for (a, b) in ab {
        ab_map
            .entry(a)
            .or_insert_with(std::collections::HashSet::new)
            .insert(b);
        ab_map
            .entry(b)
            .or_insert_with(std::collections::HashSet::new)
            .insert(a);
    }
    for (c, d) in cd {
        cd_map
            .entry(c)
            .or_insert_with(std::collections::HashSet::new)
            .insert(d);
        cd_map
            .entry(d)
            .or_insert_with(std::collections::HashSet::new)
            .insert(c);
    }

    let per = per::make_permutation(n);

    for v in per {
        let mut is_ok = true;
        for (i, value) in v.iter().enumerate() {
            let ab_set = ab_map.get(&(i + 1));
            let cd_set = cd_map.get(&(*value + 1));
            if ab_set.is_none() && cd_set.is_none() {
                continue;
            }

            if ab_set.xor(cd_set).is_some()
                || *ab_set.unwrap()
                    != cd_set
                        .unwrap()
                        .iter()
                        .map(|x| v[x - 1] + 1)
                        .collect::<std::collections::HashSet<_>>()
            {
                is_ok = false;

                break;
            }
        }

        if is_ok {
            println!("Yes");
            return;
        }
    }
    println!("No");
}

mod per {
    //! 順列生成

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
        let block_size = (1..=nums.len() - 1).product();
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
}
