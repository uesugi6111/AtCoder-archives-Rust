#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
fn main() {
    input!(h: usize, w: usize, s: [Chars; h]);

    let mut memo = vec![vec![-1isize; w]; h];
    memo[0][0] = 1;

    let ans = solve(h - 1, w - 1, &s, &mut memo);
    println!("{}", ans);
}

fn solve(x: usize, y: usize, s: &Vec<Vec<char>>, mut m: &mut Vec<Vec<isize>>) -> usize {
    if m[x][y] != -1 {
        return m[x][y] as usize;
    }

    let points = get_points(x, y, s);

    if points.is_empty() {
        return 0;
    }

    let mut count = 0;
    for i in points {
        let mut c = 0;
        if m[i.0][i.1] == -1 {
            c = solve(i.0, i.1, s, &mut m) % 1_000_000_007;
            m[i.0][i.1] = c as isize;
        } else {
            c = m[i.0][i.1] as usize;
        }

        count += c;
    }

    count
}

fn get_points(x: usize, y: usize, s: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
    let mut arr = vec![];
    for i in 1..=x {
        if s[x - i][y] == '#' {
            break;
        }
        arr.push((x - 1, y));
    }
    for i in 1..=y {
        if s[x][y - 1] == '#' {
            break;
        }
        arr.push((x, y - 1));
    }

    for i in 1..=std::cmp::min(x, y) {
        if s[x - 1][y - 1] == '#' {
            break;
        }
        arr.push((x - 1, y - 1));
    }

    arr
}
