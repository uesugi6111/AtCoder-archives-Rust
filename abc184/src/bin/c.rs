#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
fn main() {
    input!(rc: (i64, i64), tar_rc: (i64, i64));
    if rc.0 == tar_rc.0 && rc.1 == tar_rc.1 {
        println!("0");
        return;
    }

    if rc.0 + rc.1 == tar_rc.0 + tar_rc.1
        || rc.0 - rc.1 == tar_rc.0 - tar_rc.1
        || (rc.0 - tar_rc.0).abs() + (rc.1 - tar_rc.1).abs() <= 3
    {
        println!("1");
        return;
    }

    if (rc.0 == tar_rc.0 || rc.1 == tar_rc.1)
        && ((rc.0 - tar_rc.0).abs() + (rc.1 - tar_rc.1).abs()).abs() < 6
    {
        println!("2");
        return;
    }

    if ((rc.0 - tar_rc.0).abs() - (rc.1 - tar_rc.1).abs()).abs() <= 3 {
        println!("2");
        return;
    }
    if (rc.0 - tar_rc.0).abs() % 2 == (rc.1 - tar_rc.1).abs() % 2 {
        println!("2");
        return;
    }

    println!("3");
}
