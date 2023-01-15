use itertools::Itertools;

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
    let mut sc = io_pro::Scanner::new(std::io::stdin().lock());
    input!(sc = sc, n: usize, mut s: Chars, q: usize);

    let mut ss = s
        .iter()
        .copied()
        .sorted()
        .collect::<std::collections::VecDeque<_>>();

    for _ in 0..q {
        input!(sc = sc, t: usize);
        if t == 1 {
            input!(sc = sc, x: usize, c: char);

            let buff = s[x - 1];
            s[x - 1] = c;

            let bs = bs::BinarySearch::new(&ss, 0, n as i64);
            let ans = bs.search(|x, i| x[i as usize] < c) as usize;
            drop(bs);
            ss.insert(ans, c);
            //ss[ans as usize] = c;

            let bs = bs::BinarySearch::new(&ss, 0, n as i64);
            let t_index = bs.search(|x, i| x[i as usize] < buff) as usize;
            ss.remove(t_index);
        } else {
            input!(sc = sc, l: usize, r: usize);

            let ans = s[(l - 1)..r].iter().is && rh::rolling_hash(&s[(l - 1)..r], &ss);
            println!("{}", if ans { "Yes" } else { "No" });
        }
    }
}
mod rh {
    //! ロリハ
    //const BASE: u128 = 2_305_843_009_213_693_951; // 2^61-1
    const BASE: u128 = 1_000_000_007;
    pub fn rolling_hash(s: &[char], t: &std::collections::VecDeque<char>) -> bool {
        let l = s.len();

        let pow_b = BASE.wrapping_pow(l as u32);

        let mut target_hash: u128 = 0;
        let mut base_hash: u128 = 0;
        for k in 0..l {
            base_hash = base_hash.wrapping_mul(BASE) + unsafe { *s.get_unchecked(k) } as u128;
            target_hash = target_hash.wrapping_mul(BASE) + t[k] as u128;
        }
        if target_hash == base_hash {
            return true;
        }
        for k in 0..t.len() - l {
            target_hash = target_hash
                .wrapping_mul(BASE)
                .wrapping_add(t[l + k] as u128)
                .wrapping_sub((t[k] as u128).wrapping_mul(pow_b));
            if target_hash == base_hash {
                return true;
            }
        }
        false
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_rolling_hash() {
            let mut mached = false;
            let s = "hhggggghhhhhgghhhhhghgggggghggggghgghhggghhggghhhhhgggghhggggghhghghghhhhhhhgggghhhgghgggghhhggghhhgghhghhhghhghhhhhghghhghghggghhhhhghhgghghghhhhhghhhhgghhhhhghghgggggghhgghgghhghhgghghghghhghhhhhggggghggggghggggghhhhgggggghghgghhhhhhggghhhggghhghghhghhhhhhghghghhhghhhhgghhghghghgggghhghhhhgghhghghgghggghgghggggghggghhggghggghghhhgghhgggghhghhhhhghgghhhhghghhggghggghggghhhghhhgggghhhhghggghggghhggggghhghhhhhhhhggghgghhghhhhhgghhhhghhgghgghghgghghgghhgghggghghgghgghghggghghhghghhhhghhhhgghhghghhhhhghghhghghghgghhhghgghhhhhgggghhhhghghghgghhhghhhhhhhhhhhhghghghgghhhggghhhgghhhgghgghghggghhhgghhgggghhggghhghhghhhhghgghhgggghhghghhgghggggghgggghhghghhgghggggggggghhggghhhhghhhhghhggggggghhhhghhgghhhggghhghhghgghhhgghhhghgghhhhhhgghhhhgggghgghhhhhghghhhhgghhggghggghhgggghghhgghghhhgghhhhghghghggghhgggghhhhgghghhhhhhhhgghhhhgggghhgghhhghggghhghghggghhhghghhghgghggghhghhghgghghghhhgggggghghggghhhhgghhhhghghhgggghghhghgghghhhgghghhhhhghghghhgghhhhghhhgghghggggghghhhgghhhhghghhgghhghhhgghgghhhghghhgghggghhghghhgghgggghghhhhghhggghhhhhhggghgghhhghggggghhgggghhghhgghhgghhhhhghhhhhhgghhghhhhhghghghghghhggggggghghgghggghhhhhgghghhhghgghgghhhghghhhhggggghhghghghhgghghgghhgggggggghhhhhghghhhghghhhhhgghhgghhghgghghggghhgghghghhghghhghhhhghhhgggghghhghhhghhhhghhgghggghghghhghghhhggghhhgggghhggghhghgggghhhghghhhghhhhhhhghhgggghhhhgghggghhgghhhhgggghgggghhhhghhhggghhhgggggghggghgghghhhghggghghgghgggghhghhhghhgghghhhhghhgghhgghhhhhggghggghhghhghhgggghghgggghgghggghhhhhhghhghgghghhhhhghghhhggghgghghghgggghghhgghhhgghhgggggghggghhggghhhghhghghggghhggghghhhhhgghhgghhhhggghhhhgggghhhgghhggghghhghghghghhgghghggghgghgghghhgghhhghgghghhggghghhghhhhgghhghghhhghghhghghghhgghhhgghghhghggggghhhhhggggggggghhhhgghhhgghhghghggghghghhgggghghghhhgghgghhghhgghggggggghggghggghghgggghghhggghgghhhhhhgghhghggghgghhhhghhghghhhggggghhghhgghhgghhgghggghggghhhhhgghgggggghghhhhhghhhghhhghghghhhhhghggghhgghhhhhgggghhghhhhhghhgghhhgghgggghghgghggghhhhhhhhggghghghhhghhghhgghhghghhhgghghgghhhhggghhhggghhgghghgghhhhhggghhghghghghhgghghghhghghghhgggggghghhhghhhhhggghgghghhgghghhhhghhhhgghhghghhggggghghghghghhghgghggghhghhggghggghhggghhgghghhghggghgggghgghhhghgghgghhhghghghhghhhggghhhhgghhhhgghhhghghhghggggggghhhhghghhhgghhggghhghgghgghhhghhhghggghhggghghhgghhhghgghhhghhgghghhhghgghggghgghhghhggghhghhhhghgghgghggggghgghhhhhhghhgggggggghggggghgggggggghghhhhghhghggghgggghghghhghgghhgghhgghghhhghhghhghhgghgghhhgghghhhhgghghgghhgggghggggggghghghhghgggghghhgggghgghghhhhhgghhghgghghhggghghhghhghgghhggghhhhgghhhhhgggggghhggghhghhggghgghhghhhghhhhhgghhghgggggghgghggghhghgghghhgggggghgghhhghhhhghgghhhhhhghghhgghgghhgghhgghhhhhggggghhghhgghhghgghghghhhghgghggghghghgggghgghhggghghgghghhhhhhgggggghghghhhghhghhhgghghghgghhhghhggghggggggggghhhgghghhhhhhghgghgghgghghgghhhhhhgggghghhhhgggghgggggghhhghghghgghhghghhhhghgghhhhhgggggggggghghghggggggghgghhgghghhghhhhggghhghgggghghhghgghgghggghhhgghhhghgghhhhhghggghhhhghhghhhhghhggggghhhhhhhhghhhghghhggghhhhgggggghghhhgggghghhhgghhggghhghhghhgghggggggghhhgghgghghhhghgghhhghhhhhgghgghhhhgghhhhhhghhggghhhgghggggghghghghgghghgghhhhhhhhhhgghhhgghgghghhghhghgghgghggghghggghhhgghgghhhghghgghghghhhgghhggghhggggggghgghghhghghghhhhghhgghhhgghghhhghhghhhhhghhhgghgghhhghhhhghhhghghgghhghhgggggghgghghghhhghgghhhhhhhhhghghhhhhggggghgggghhhghgghhhghhhghhgghghhghggghggghhghgghhhghghhhhhggghhhghghhhgghhhhgghgggghhhghgghhggghhhggggghghhhgggghghgghhggghgg".chars().collect::<Vec<_>>();

            for i in 0..s.len() {
                if i + 21 * 2 > s.len() {
                    break;
                }
                if rolling_hash(&s[i..i + 21], &s) {
                    mached = true;
                    break;
                }
            }
            assert!(mached);
        }
    }
}

mod bs {
    pub struct BinarySearch<T> {
        target: T,
        min: i64,
        max: i64,
    }
    impl<T> BinarySearch<T> {
        pub fn new(target: T, min: i64, max: i64) -> Self {
            Self { target, min, max }
        }
        /// f が true を帰す最小値を探す
        pub fn search<F>(&self, f: F) -> i64
        where
            F: Fn(&T, i64) -> bool,
        {
            let mut left = self.min;
            let mut right = self.max;

            while right - left > 1 {
                let mid = left + (right - left) / 2;

                if f(&self.target, mid) {
                    left = mid;
                } else {
                    right = mid
                };
            }
            right
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        #[test]
        fn test_binary_search() {
            let v = (0_i64..100).filter(|x| x % 2 == 0).collect::<Vec<_>>();
            let bs = BinarySearch::new(&v, -1, v.len() as i64);
            for &i in v.iter() {
                let ans = bs.search(|x, j| x[j as usize] < i);
                assert_eq!(v[ans as usize], i)
            }
        }
    }
}
