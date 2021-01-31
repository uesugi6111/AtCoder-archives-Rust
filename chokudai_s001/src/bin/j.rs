#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
fn main() {
    input!(n: usize, a: [i64; n]);

    println!("{}", inversionnumber::inversion_number(&a));
}

mod inversionnumber {
    pub fn inversion_number(array: &[i64]) -> i64 {
        count_merge(
            &mut array.iter().copied().collect::<Vec<_>>(),
            0..array.len(),
        )
    }
    fn count_merge(array: &mut Vec<i64>, range: std::ops::Range<usize>) -> i64 {
        let length = range.len() as i64;
        if length <= 1 {
            return 0;
        }

        let mut count = 0;
        let mid = (range.start + range.end) / 2;
        count += count_merge(array, range.start..mid);
        count += count_merge(array, mid..range.end);

        let b = array
            .iter()
            .skip(range.start)
            .take(mid - range.start)
            .copied()
            .collect::<Vec<_>>();
        let c = array
            .iter()
            .skip(mid)
            .take(range.end - mid)
            .copied()
            .collect::<Vec<_>>();

        let (mut ai, mut bi, mut ci) = (0, 0, 0);

        while ai < length {
            if bi < b.len() && (ci == c.len() || b[bi] <= c[ci]) {
                array[range.start + ai as usize] = b[bi];
                ai += 1;
                bi += 1;
            } else {
                count += length / 2 - bi as i64;
                array[range.start + ai as usize] = c[ci];
                ai += 1;
                ci += 1;
            }
        }
        count
    }
}
