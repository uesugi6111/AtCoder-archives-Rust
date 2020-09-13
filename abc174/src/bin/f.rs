mod fenwicktree {
    use std::clone::Clone;
    use std::convert::From;
    use std::ops::{Add, AddAssign, Sub};

    #[derive(Clone, Debug)]
    pub struct FenwickTree<T> {
        array: Vec<T>,
    }

    impl<T> FenwickTree<T>
    where
        T: Add + Sub + Clone + Copy + From<u8> + AddAssign,
    {
        pub fn new(size: usize) -> FenwickTree<T> {
            let v: Vec<T> = vec![T::from(0u8); size + 1];
            Self { array: v }
        }
        pub fn add(&mut self, mut i: usize, x: T) {
            let n = self.array.len();
            while i <= n {
                self.array[i] += x;
                i += i & i.wrapping_neg();
            }
        }
    }

    pub trait Sum<T, U> {
        fn sum(&self, i: T) -> U;
    }

    impl<T> Sum<usize, T> for FenwickTree<T>
    where
        T: Add + Sub + Clone + Copy + From<u8> + AddAssign,
    {
        fn sum(&self, mut i: usize) -> T {
            if i == 0 {
                return T::from(0u8);
            }
            let mut s = T::from(0u8);

            while i > 0 {
                s += self.array[i];
                i -= i & i.wrapping_neg();
            }
            s
        }
    }

    impl<T> Sum<(usize, usize), T> for FenwickTree<T>
    where
        T: Add + Sub + Clone + Copy + From<u8> + AddAssign,
        T: std::ops::Sub<Output = T>,
    {
        fn sum(&self, i: (usize, usize)) -> T {
            let sum_l = <FenwickTree<T> as Sum<usize, T>>::sum(self, i.0 - 1);
            let sum_r = <FenwickTree<T> as Sum<usize, T>>::sum(self, i.1);
            sum_r - sum_l
        }
    }

    #[test]
    fn test_sum() {
        let mut a = FenwickTree::new(100);

        for i in 1..101 {
            a.add(i, i);
        }

        assert_eq!((0..101).sum::<usize>(), a.sum(100));
        assert_eq!((2..101).sum::<usize>(), a.sum((2, 100)));
    }
}

use fenwicktree::Sum;
use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input!(n: usize, q: usize, c: [usize; n], lr: [[Usize1; 2]; q]);

    let mut c_vv = vec![vec![0usize; 0]; n];
    let mut c_v = vec![-1i64; n + 1];
    for (i, v) in c.iter().enumerate() {
        let l = c_v[*v];
        if l != -1 {
            c_vv[l as usize].push(i);
        }
        c_v[*v] = i as i64;
    }

    let mut lr_vec = vec![vec![vec![0usize; 2]; 0]; n];
    for (i, v) in lr.iter().enumerate() {
        lr_vec[v[0]].push(vec![v[1], i]);
    }

    let mut ans = vec![0usize; q];
    let mut bi_tree = fenwicktree::FenwickTree::<usize>::new(n);
    for i in (0..n).rev() {
        for cv in &c_vv[i] {
            bi_tree.add(*cv + 1, 1);
        }

        for v in &lr_vec[i] {
            ans[v[1]] = (v[0] - i + 1) - bi_tree.sum(v[0] + 1);
        }
    }
    for v in &ans {
        println!("{}", *v);
    }
}
