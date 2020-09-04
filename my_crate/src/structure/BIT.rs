use std::clone::Clone;
use std::convert::From;
use std::ops::{Add, AddAssign, Sub};

#[derive(Clone, Debug)]
pub struct BIT<T> {
    array: Vec<T>,
}

impl<T> BIT<T>
where
    T: Add + Sub + Clone + Copy + From<u8> + AddAssign,
{
    pub fn new(size: usize) -> BIT<T> {
        let v: Vec<T> = vec![T::from(0u8); size + 1];
        Self { array: v }
    }
    pub fn add(&mut self, i: usize, x: T) {
        let mut cnt = i as i64;
        let n = self.array.len() as i64;
        while cnt <= n {
            self.array[cnt as usize] += x;
            cnt += cnt & -cnt;
        }
    }
}

trait Sum<T, U> {
    fn sum(&self, i: T) -> U;
}

impl<T> Sum<usize, T> for BIT<T>
where
    T: Add + Sub + Clone + Copy + From<u8> + AddAssign,
{
    fn sum(&self, i: usize) -> T {
        let mut s = T::from(0u8);

        let mut cnt = i as i64;
        while cnt > 0 {
            s += self.array[cnt as usize];
            cnt -= cnt & -cnt;
        }
        s
    }
}

// impl<T> Sum<(usize, usize), T> for BIT<T>
// where
//     T: Add + Sub + Clone + From<i32> + AddAssign,
//     T: std::ops::Sub<Output = T>,
// {
//     fn sum(&self, i: (usize, usize)) -> T {
//         let sum_l = self.sum(i.0 - 1);
//         let sum_r = self.sum(i.1);
//         sum_r - sum_l
//     }
// }

#[test]
fn test_sum() {
    let mut a = BIT::new(100);

    for i in 1..101 {
        a.add(i, i);
    }
    let sum = a.sum(100);

    assert_eq!((0..101).sum::<usize>(), sum);
}
