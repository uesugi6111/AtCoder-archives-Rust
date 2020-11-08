#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
fn main() {
    input!(n: usize, m: usize);

    let mut list = vec![];

    for _ in 0..m {
        input!(k: usize, s: [Usize1; k]);

        list.push(s);
    }

    input!(p: [usize; m]);

    let mut count = 0;

    for i in 0..1 << n {
        let mut pp = vec![0usize; n];
        for (j, ppp) in pp.iter_mut().enumerate() {
            if (1 << j) & i != 0 {
                *ppp += 1;
            }
        }

        let mut on = true;

        for (ii, j) in list.iter().enumerate() {
            let mut sum = 0;
            for k in j {
                sum += pp[*k];
            }
            if sum % 2 != p[ii] {
                on = false;
                break;
            }
        }

        if on {
            count += 1;
        }
    }

    println!("{}", count);
}
