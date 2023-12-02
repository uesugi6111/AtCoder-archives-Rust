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
    input!(n: usize, m: usize, s: Chars, t: Chars);

    let rh = rh::rolling_hash(&t, &s);
    let mut start = 0;

    for end in rh {
        let mut is_ok = true;
        let mut m_index = 0;
        for i in (start..end + m) {
            if t[m_index % m] == s[i] {
                m_index += 1;
                m_index %= m;
                continue;
            } else if t[0] == s[i] {
                m_index = 1;
                continue;
            } else {
                is_ok = false;
                break;
            }
        }

        if is_ok {
            start = end;
            continue;
        }
        is_ok = true;

        for i in (start..end + m).rev() {
            if t[(m + m - 1 - m_index) % m] == s[i] {
                m_index += 1;
                m_index %= m;
                continue;
            } else if t[m - 1] == s[i] {
                m_index = 1;
                continue;
            } else {
                is_ok = false;
                break;
            }
        }
        if !is_ok {
            println!("No");
            return;
        }
        start = end;
    }

    let mut is_ok = true;
    let mut m_index = 0;
    for i in (start..n) {
        if t[m_index % m] == s[i] {
            m_index += 1;
            continue;
        } else if t[0] == s[i] {
            m_index = 1;
            continue;
        } else {
            is_ok = false;
            break;
        }
    }

    if is_ok {
        println!("Yes");
        return;
    }
    is_ok = true;

    for i in (start..n).rev() {
        if t[(m + m - 1 - m_index) % m] == s[i] {
            m_index += 1;
            continue;
        } else if t[m - 1] == s[i] {
            m_index = 1;
            continue;
        } else {
            is_ok = false;
            break;
        }
    }
    if is_ok {
        println!("Yes");
    } else {
        println!("No");
    }
}

mod xorshift {
    //! Xorshift random number generator
    use std::{
        fmt::{Debug, Display},
        time::SystemTime,
    };

    #[derive(Clone, Default, Copy, Debug)]
    pub struct XorShift<T>
    where
        T: std::fmt::Debug + Sized + Copy + Display + Shift,
    {
        seed: T,
    }

    impl<T> XorShift<T>
    where
        T: std::fmt::Debug + Sized + Copy + Display + Shift,
    {
        pub fn new() -> Self {
            XorShift::from_seed(T::seed())
        }
        pub fn from_seed(seed: T) -> XorShift<T> {
            XorShift { seed }
        }
    }

    impl<T> Iterator for XorShift<T>
    where
        T: std::fmt::Debug + Sized + Copy + Display + Shift,
    {
        type Item = T;

        fn next(&mut self) -> Option<Self::Item> {
            T::shift(&mut self.seed);
            Some(self.seed)
        }
    }

    pub trait Shift {
        fn seed() -> Self;
        fn shift(n: &mut Self);
    }

    impl Shift for u64 {
        fn seed() -> Self {
            SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs()
        }

        fn shift(state: &mut u64) {
            *state ^= *state << 13;
            *state ^= *state >> 7;
            *state ^= *state << 17;
        }
    }
    impl Shift for u32 {
        fn seed() -> Self {
            SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs() as u32
        }

        fn shift(state: &mut u32) {
            *state ^= *state << 13;
            *state ^= *state >> 17;
            *state ^= *state << 5;
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use std::collections::HashSet;
        #[test]
        fn test_xorshift() {
            let mut set = HashSet::new();
            let xorshift = XorShift::<u64>::new();

            for v in xorshift.take(100_000) {
                assert!(!set.contains(&v));
                set.insert(v);
            }
        }
    }
}
mod rh {
    //! ロリハ
    use super::xorshift::XorShift;

    const MOD: u128 = (1 << 61) - 1;

    ///  text 内での pattern の出現位置の始点を返す
    pub fn rolling_hash<T: Into<u128> + Copy>(pattern: &[T], text: &[T]) -> Vec<usize> {
        let rh = RollingHash::new(text, pattern.len());
        rh.search(pattern)
    }

    pub struct RollingHash {
        pub text_hash: Vec<u128>,
        base: u128,
        length: usize,
    }
    impl RollingHash {
        pub fn new<T: Into<u128> + Copy>(text: &[T], length: usize) -> Self {
            let base = XorShift::new()
                .map(|x: u64| x as u128 % MOD)
                .next()
                .unwrap();
            RollingHash::from_base(text, length, base)
        }
        pub fn from_base<T: Into<u128> + Copy>(text: &[T], length: usize, base: u128) -> Self {
            let pow_base = pow_mod(base, length);

            let mut hash = 0;

            for &t in text.iter().take(length) {
                hash = mul_mod(hash, base);

                hash += t.into();
            }
            let mut text_hash = vec![hash];

            for k in 0..text.len() - length {
                hash = mul_mod(hash, base);
                hash += text[length + k].into();

                let buff = mul_mod(text[k].into(), pow_base);
                if buff < hash {
                    hash -= buff;
                } else {
                    hash += MOD - buff;
                }

                text_hash.push(hash);
            }
            Self {
                text_hash,
                base,
                length,
            }
        }
        pub fn search<T: Into<u128> + Copy>(&self, pattern: &[T]) -> Vec<usize> {
            assert_eq!(self.length, pattern.len());

            let mut pattern_hash = 0;

            for &p in pattern {
                pattern_hash = mul_mod(pattern_hash, self.base);
                pattern_hash += p.into();
            }

            self.text_hash
                .iter()
                .enumerate()
                .filter_map(|(i, &h)| if pattern_hash == h { Some(i) } else { None })
                .collect()
        }
    }

    fn mul_mod(a: u128, b: u128) -> u128 {
        let mut t = a * b;
        t = (t >> 61) + (t & MOD);

        if t >= MOD {
            t - MOD
        } else {
            t
        }
    }
    fn pow_mod(base: u128, exp: usize) -> u128 {
        let (mut a, mut exp) = (base, exp as u128);

        if exp == 0 {
            return 1;
        }

        let mut res = 1;
        a %= MOD;

        loop {
            if exp % 2 == 1 {
                res = mul_mod(res, a);
            }

            if exp == 1 {
                return res;
            }

            exp /= 2;
            a = mul_mod(a, a);
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_rolling_hash() {
            let mut mached = false;
            let s = b"hhggggghhhhhgghhhhhghgggggghggggghgghhggghhggghhhhhgggghhggggghhghghghhhhhhhgggghhhgghgggghhhggghhhgghhghhhghhghhhhhghghhghghggghhhhhghhgghghghhhhhghhhhgghhhhhghghgggggghhgghgghhghhgghghghghhghhhhhggggghggggghggggghhhhgggggghghgghhhhhhggghhhggghhghghhghhhhhhghghghhhghhhhgghhghghghgggghhghhhhgghhghghgghggghgghggggghggghhggghggghghhhgghhgggghhghhhhhghgghhhhghghhggghggghggghhhghhhgggghhhhghggghggghhggggghhghhhhhhhhggghgghhghhhhhgghhhhghhgghgghghgghghgghhgghggghghgghgghghggghghhghghhhhghhhhgghhghghhhhhghghhghghghgghhhghgghhhhhgggghhhhghghghgghhhghhhhhhhhhhhhghghghgghhhggghhhgghhhgghgghghggghhhgghhgggghhggghhghhghhhhghgghhgggghhghghhgghggggghgggghhghghhgghggggggggghhggghhhhghhhhghhggggggghhhhghhgghhhggghhghhghgghhhgghhhghgghhhhhhgghhhhgggghgghhhhhghghhhhgghhggghggghhgggghghhgghghhhgghhhhghghghggghhgggghhhhgghghhhhhhhhgghhhhgggghhgghhhghggghhghghggghhhghghhghgghggghhghhghgghghghhhgggggghghggghhhhgghhhhghghhgggghghhghgghghhhgghghhhhhghghghhgghhhhghhhgghghggggghghhhgghhhhghghhgghhghhhgghgghhhghghhgghggghhghghhgghgggghghhhhghhggghhhhhhggghgghhhghggggghhgggghhghhgghhgghhhhhghhhhhhgghhghhhhhghghghghghhggggggghghgghggghhhhhgghghhhghgghgghhhghghhhhggggghhghghghhgghghgghhgggggggghhhhhghghhhghghhhhhgghhgghhghgghghggghhgghghghhghghhghhhhghhhgggghghhghhhghhhhghhgghggghghghhghghhhggghhhgggghhggghhghgggghhhghghhhghhhhhhhghhgggghhhhgghggghhgghhhhgggghgggghhhhghhhggghhhgggggghggghgghghhhghggghghgghgggghhghhhghhgghghhhhghhgghhgghhhhhggghggghhghhghhgggghghgggghgghggghhhhhhghhghgghghhhhhghghhhggghgghghghgggghghhgghhhgghhgggggghggghhggghhhghhghghggghhggghghhhhhgghhgghhhhggghhhhgggghhhgghhggghghhghghghghhgghghggghgghgghghhgghhhghgghghhggghghhghhhhgghhghghhhghghhghghghhgghhhgghghhghggggghhhhhggggggggghhhhgghhhgghhghghggghghghhgggghghghhhgghgghhghhgghggggggghggghggghghgggghghhggghgghhhhhhgghhghggghgghhhhghhghghhhggggghhghhgghhgghhgghggghggghhhhhgghgggggghghhhhhghhhghhhghghghhhhhghggghhgghhhhhgggghhghhhhhghhgghhhgghgggghghgghggghhhhhhhhggghghghhhghhghhgghhghghhhgghghgghhhhggghhhggghhgghghgghhhhhggghhghghghghhgghghghhghghghhgggggghghhhghhhhhggghgghghhgghghhhhghhhhgghhghghhggggghghghghghhghgghggghhghhggghggghhggghhgghghhghggghgggghgghhhghgghgghhhghghghhghhhggghhhhgghhhhgghhhghghhghggggggghhhhghghhhgghhggghhghgghgghhhghhhghggghhggghghhgghhhghgghhhghhgghghhhghgghggghgghhghhggghhghhhhghgghgghggggghgghhhhhhghhgggggggghggggghgggggggghghhhhghhghggghgggghghghhghgghhgghhgghghhhghhghhghhgghgghhhgghghhhhgghghgghhgggghggggggghghghhghgggghghhgggghgghghhhhhgghhghgghghhggghghhghhghgghhggghhhhgghhhhhgggggghhggghhghhggghgghhghhhghhhhhgghhghgggggghgghggghhghgghghhgggggghgghhhghhhhghgghhhhhhghghhgghgghhgghhgghhhhhggggghhghhgghhghgghghghhhghgghggghghghgggghgghhggghghgghghhhhhhgggggghghghhhghhghhhgghghghgghhhghhggghggggggggghhhgghghhhhhhghgghgghgghghgghhhhhhgggghghhhhgggghgggggghhhghghghgghhghghhhhghgghhhhhgggggggggghghghggggggghgghhgghghhghhhhggghhghgggghghhghgghgghggghhhgghhhghgghhhhhghggghhhhghhghhhhghhggggghhhhhhhhghhhghghhggghhhhgggggghghhhgggghghhhgghhggghhghhghhgghggggggghhhgghgghghhhghgghhhghhhhhgghgghhhhgghhhhhhghhggghhhgghggggghghghghgghghgghhhhhhhhhhgghhhgghgghghhghhghgghgghggghghggghhhgghgghhhghghgghghghhhgghhggghhggggggghgghghhghghghhhhghhgghhhgghghhhghhghhhhhghhhgghgghhhghhhhghhhghghgghhghhgggggghgghghghhhghgghhhhhhhhhghghhhhhggggghgggghhhghgghhhghhhghhgghghhghggghggghhghgghhhghghhhhhggghhhghghhhgghhhhgghgggghhhghgghhggghhhggggghghhhgggghghgghhggghgg";

            for i in 0..s.len() {
                if i + 21 * 2 > s.len() {
                    break;
                }
                if !rolling_hash(&s[i..i + 21], s).is_empty() {
                    mached = true;
                    break;
                }
            }
            assert!(mached);
        }

        #[test]
        fn abc() {
            let s = b"abcdefghijklmnopqrstuvwxyz";
            assert_eq!(rolling_hash(s, s), vec![0]);

            for i in 1..26 {
                for j in 0..26 - i {
                    assert_eq!(rolling_hash(&s[j..j + i], s), vec![j], "{} {}", i, j);
                }
            }
        }

        #[test]
        fn test_mul_mod() {
            for i in XorShift::new().take(1000).map(|x: u64| x as u128 % MOD) {
                for j in XorShift::new().take(1000).map(|x: u64| x as u128 % MOD) {
                    assert_eq!(mul_mod(i, j), (i * j) % MOD);
                }
            }
        }
        #[test]
        fn test_pow_mod() {
            use crate::math::mod_pow::modpow;
            for i in XorShift::new().take(1000).map(|x: u64| x as u128 % MOD) {
                for j in XorShift::new().take(1000).map(|x: u64| x as u128 % MOD) {
                    assert_eq!(
                        pow_mod(i, j as usize),
                        modpow(i as i64, j as i64, MOD as i64) as u128
                    );
                }
            }
        }
    }
}
