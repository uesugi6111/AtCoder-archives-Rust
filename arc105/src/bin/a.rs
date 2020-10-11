#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
fn main() {
    input!(a: [usize; 4]);

    let sum = a.iter().sum::<usize>();

    for v in &a {
        if sum == v * 2 {
            println!("Yes");
            return;
        }
    }

    if 2 * (a[0] + a[1]) == sum {
        println!("Yes");
        return;
    }
    if 2 * (a[0] + a[2]) == sum {
        println!("Yes");
        return;
    }
    if 2 * (a[0] + a[3]) == sum {
        println!("Yes");
        return;
    }

    println!("No");
}
