#[allow(unused_imports)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(n: usize, a: [f64; n]);

    let aaa = 1e9f64;
    let mut vvv: Vec<Vec<_>> = Vec::new();
    for v in a {
        let buff = (v * aaa + 0.5) as usize;
        vvv.push(count_div2and5(buff));
    }

    let mut gurahu = vec![vec![0isize; 19]; 19];
    for v in &vvv {
        gurahu[v[0]][v[1]] += 1;
    }

    let mut count: isize = 0;
    for i in 0..n {
        for j in 0..19 {
            for k in 0..19 {
                if vvv[i][0] + j >= 18 && vvv[i][1] + k >= 18 {
                    count += gurahu[j][k];
                }
            }
        }
        if vvv[i][0] * 2 >= 18 && vvv[i][1] * 2 >= 18 {
            count -= 1;
        }
    }

    println!("{}", count / 2);
}

fn count_div2and5(target: usize) -> Vec<usize> {
    let mut count2: usize = 0;
    let mut count5: usize = 0;
    let mut buff = target;
    loop {
        if buff % 2 == 0 && count2 != 18 {
            buff /= 2;
            count2 += 1;
        } else {
            break;
        }
    }
    loop {
        if buff % 5 == 0 && count5 != 18 {
            buff /= 5;
            count5 += 1;
        } else {
            break;
        }
    }

    return vec![count2, count5];
}
