#[allow(unused_imports)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(x: usize);

    let mut bool_array = vec![true; 100005];
    let max_f = 100000.0_f64;

    for i in 2..max_f.sqrt().floor() as usize {
        if !bool_array[i] {
            continue;
        }
        for j in 0..100005 {
            if i == j {
                continue;
            }
            if j % i == 0 {
                bool_array[j] = false;
            }
        }
    }

    for i in x..100005 {
        if bool_array[i] {
            println!("{}", i);
            break;
        }
    }
}
