use fenwicktree::Sum;
#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
fn main() {
    input!(n: usize, q: usize, a: [i64; n], txy: [(i32, i64, i64); q]);

    let mut ft = fenwicktree::FenwickTree::new(n + 3);

    for v in a.iter().enumerate() {
        ft.add(v.0 as i64 + 1, *v.1);
    }

    for (t, x, y) in txy {
        if t == 1 {
            ft.add(x, y);
        } else {
            println!("{}", ft.sum((x, y)));
        }
    }
}

mod fenwicktree {
    use std::clone::Clone;
    use std::convert::From;
    use std::ops::{Add, AddAssign, Sub};

    ///binaryIndexTree
    #[derive(Clone, Debug)]
    pub struct FenwickTree<T> {
        array: Vec<T>,
    }

    impl<T> FenwickTree<T>
    where
        T: Add + Sub + Clone + Copy + From<u8> + AddAssign + std::ops::BitXorAssign,
    {
        pub fn new(size: usize) -> FenwickTree<T> {
            let v: Vec<T> = vec![T::from(0u8); size + 1];
            Self { array: v }
        }
        pub fn add(&mut self, mut i: i64, x: T) {
            let n = self.array.len();
            while i as usize <= n {
                self.array[i as usize] ^= x;
                i += i & i.wrapping_neg();
            }
        }
    }

    pub trait Sum<T, U> {
        fn sum(&self, i: T) -> U;
    }

    impl<T> Sum<i64, T> for FenwickTree<T>
    where
        T: Add + Sub + Clone + Copy + From<u8> + AddAssign + std::ops::BitXorAssign,
    {
        fn sum(&self, mut i: i64) -> T {
            if i == 0 {
                return T::from(0u8);
            }
            let mut s = T::from(0u8);

            while i > 0 {
                s ^= self.array[i as usize];
                i -= i & i.wrapping_neg();
            }
            s
        }
    }

    impl<T> Sum<(i64, i64), T> for FenwickTree<T>
    where
        T: Add + Sub + Clone + Copy + From<u8> + AddAssign + std::ops::BitXorAssign,
        T: std::ops::Sub<Output = T> + std::ops::BitXor<Output = T>,
    {
        fn sum(&self, i: (i64, i64)) -> T {
            let sum_l = <FenwickTree<T> as Sum<i64, T>>::sum(self, i.0 - 1);
            let sum_r = <FenwickTree<T> as Sum<i64, T>>::sum(self, i.1);
            sum_r ^ sum_l
        }
    }
}
