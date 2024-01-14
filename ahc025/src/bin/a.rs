use std::io::{self, BufReader};

use proconio::{input, source::line::LineSource};

fn main() {
    let mut stdin = LineSource::new(BufReader::new(io::stdin()));
    macro_rules! input(($($tt:tt)*) => (proconio::input!(from &mut stdin, $($tt)*)));

    input!(n: usize, d: usize, q: usize);
    let mut xorshift = xorshift::XorShift::<u64>::new();

    let mut map = std::collections::HashMap::new();
    for i in 0..d {
        map.insert(i, std::collections::VecDeque::new());
    }
    for i in 0..n {
        map.entry(i % d)
            .or_insert(std::collections::VecDeque::new())
            .push_back(i);
    }

    for _ in 0..q {
        let l = xorshift.next().unwrap() as usize % d;
        let r = ((l + 1) + xorshift.next().unwrap() as usize % (d - 1)) % d;
        let l_len = map.get(&l).unwrap().len();
        let r_len = map.get(&r).unwrap().len();
        print!("{} {} ", l_len, r_len,);
        for j in map.get(&l).unwrap() {
            print!("{} ", j);
        }
        for j in map.get(&r).unwrap() {
            print!("{} ", j);
        }
        println!();
        input!(c: char);
        let (l, r) = if c == '>' {
            if l_len == 1 {
                continue;
            }
            (l, r)
        } else {
            if r_len == 1 {
                continue;
            }
            (r, l)
        };

        let buff = map
            .get_mut(&l)
            .unwrap()
            .remove(xorshift.next().unwrap() as usize % l_len)
            .unwrap();
        map.get_mut(&r).unwrap().push_back(buff);
    }

    let mut ans = vec![0; n];

    for (k, v) in map {
        for i in v {
            ans[i] = k;
        }
    }

    for i in 0..n {
        print!("{} ", ans[i]);
    }

    println!();
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
