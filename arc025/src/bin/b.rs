use itertools::*;
#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
fn main() {
    input!(h: usize, w: usize, mut hw: [[i64; w]; h]);

    for i in 0..h {
        for j in 0..w {
            if (i + j) % 2 == 0 {
                hw[i][j] *= -1;
            }
        }
    }

    let cumsum = cumsum2d::CumSum2D::new(&hw);

    let mut max = 0;
    for i in (0..h).combinations_with_replacement(2) {
        for j in (0..w).combinations_with_replacement(2) {
            if cumsum.query(i[0], i[1], j[0], j[1]) == 0 {
                max = std::cmp::max(max, (i[1] - i[0] + 1) * (j[1] - j[0] + 1));
            }
        }
    }

    println!("{}", max);
}

mod cumsum2d {
    #[derive(Clone, Debug)]
    pub struct CumSum2D {
        v: Vec<Vec<i64>>,
    }

    impl CumSum2D {
        pub fn new(source: &[Vec<i64>]) -> Self {
            let h = source.len();
            let w = source[0].len();
            let mut v = vec![vec![0i64; w + 1]; h + 1];

            for i in 0..h {
                for j in 0..w {
                    v[i + 1][j + 1] = source[i][j] + v[i][j + 1] + v[i + 1][j] - v[i][j];
                }
            }
            CumSum2D { v }
        }

        pub fn query(&self, top: usize, bottom: usize, left: usize, right: usize) -> i64 {
            self.v[bottom + 1][right + 1] - self.v[bottom + 1][left] - self.v[top][right + 1]
                + self.v[top][left]
        }
    }
}
