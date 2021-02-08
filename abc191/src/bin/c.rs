#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
fn main() {
    input!(h: usize, w: usize, s: [Chars; h]);

    let mut count = 0;

    for i in 0..h - 1 {
        for j in 0..w - 1 {
            let mut edge_count = 0;
            if s[i][j] == '#' {
                edge_count += 1;
            }
            if s[i + 1][j] == '#' {
                edge_count += 1;
            }
            if s[i][j + 1] == '#' {
                edge_count += 1;
            }
            if s[i + 1][j + 1] == '#' {
                edge_count += 1;
            }
            if edge_count == 1 || edge_count == 3 {
                count += 1;
            }
        }
    }
    println!("{}", count);
}
