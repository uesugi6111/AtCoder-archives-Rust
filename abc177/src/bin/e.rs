use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(n: usize, a: [usize; n]);

    const A_SIZE: usize = 1_000_002;
    let mut do_continue = true;
    let mut ans = "pairwise coprime";
    let mut aaa: Vec<usize> = vec![0usize; A_SIZE];
    for v in &a {
        aaa[*v] += 1;
    }

    for i in 2..A_SIZE {
        let mut count = 0;
        for j in (0..A_SIZE).skip(i).step_by(i) {
            count += aaa[j];
        }
        if count > 1 {
            do_continue = false;
            ans = "not coprime";
            break;
        }
    }

    if !do_continue {
        let mut c = a[0];
        for i in a.iter().skip(1) {
            c = num::integer::gcd(c, *i);
        }
        if c == 1 {
            ans = "setwise coprime";
        }
    }

    println!("{}", ans);
}
