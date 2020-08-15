#[allow(unused_imports)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(n: usize, k: usize, mut p: [usize; n], c: [isize; n]);

    for i in 0..n {
        p[i] -= 1;
    }

    let mut ans = (1 << 30) * -1;

    for i in 0..n {
        let mut pos = p[i];
        let mut sum = c[i];
        let mut v = Vec::new();
        v.push(c[i]);
        while i != pos {
            v.push(c[pos]);
            sum += c[pos];
            pos = p[pos];
        }
        let v_size = v.len();
        let mut v_r = vec![0; v_size];
        v_r[0] = v[0];
        for i in 1..v_size {
            v_r[i] = v_r[i - 1] + v[i];
        }

        for i in 0..v_r.len() {
            if i + 1 > k {
                break;
            }
            let mut buff = v_r[i];
            if 0 < sum {
                buff += sum * ((k - i - 1) / v.len()) as isize;
            }

            if ans < buff {
                ans = buff;
            }
        }
    }
    println!("{}", ans);
}
