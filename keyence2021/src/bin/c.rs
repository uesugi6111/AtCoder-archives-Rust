#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
fn main() {
    input!(h: usize, w: usize, k: usize, hwc: [(i64, i64, char); k]);

    let mut grid = vec![vec!['*'; w + 1]; h + 1];
    for v in hwc {
        grid[v.0 as usize][v.1 as usize] = v.2;
    }
    (0..w + 1).for_each(|i| grid[0][i] = '-');
    (0..h + 1).for_each(|i| grid[i][0] = '-');

    let mut memo = vec![vec![-1; w + 1]; h + 1];
    memo[1][1] = 1;

    let mut sum = excute(h, w, &grid, &mut memo);

    if grid[h][w] == '*' {
        sum *= 3;
        sum %= 998_244_353;
    }

    println!("{}", sum);
}

fn excute(h: usize, w: usize, grid: &[Vec<char>], memo: &mut Vec<Vec<i64>>) -> i64 {
    if memo[h][w] != -1 {
        return memo[h][w];
    }
    let mut ans = 0;

    if w > 1 && grid[h][w - 1] != 'D' {
        let buff = excute(h, w - 1, &grid, memo);

        ans += buff
            * if grid[h - 1][w] == '*' { 3 } else { 1 }
            * if grid[h][w - 1] == '*' { 2 } else { 1 };
    }
    if h > 1 && grid[h - 1][w] != 'R' {
        let buff = excute(h - 1, w, &grid, memo);

        ans += buff
            * if grid[h][w - 1] == '*' { 3 } else { 1 }
            * if grid[h - 1][w] == '*' { 2 } else { 1 };
    }

    ans %= 998_244_353;

    memo[h][w] = ans;
    ans
}
