use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(
        n: usize,
        m: usize,
        c: isize,
        b: [isize; m],
        a: [[isize; m]; n]
    );

    let mut count = 0;
    for i in 0..n {
        let mut buff = c;
        for j in 0..m {
            buff += b[j] * a[i][j];
        }
        if buff > 0 {
            count += 1;
        }
    }

    println!("{}", count);
}
