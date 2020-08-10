#[allow(unused_imports)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(sa: String, sb: String, sc: String);
    let mut a: Vec<char> = sa.chars().collect();
    let mut b: Vec<char> = sb.chars().collect();
    let mut c: Vec<char> = sc.chars().collect();

    let mut buff = a[0];
    a.remove(0);
    loop {
        if buff == 'a' {
            if a.len() == 0 {
                println!("{}", "A");
                break;
            }
            buff = a[0];

            a.remove(0);
        } else if buff == 'b' {
            if b.len() == 0 {
                println!("{}", "B");
                break;
            }
            buff = b[0];

            b.remove(0);
        } else if buff == 'c' {
            if c.len() == 0 {
                println!("{}", "C");
                break;
            }
            buff = c[0];

            c.remove(0);
        }
    }
}
