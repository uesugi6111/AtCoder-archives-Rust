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
            buff = a[0];
            a.remove(0);
        } else if buff == 'b' {
            buff = b[0];
            b.remove(0);
        } else if buff == 'c' {
            buff = c[0];
            c.remove(0);
        }

        if a.is_empty() {
            println!("A");
            break;
        }
        if b.is_empty() {
            println!("B");
            break;
        }
        if c.is_empty() {
            println!("C");
            break;
        }
    }
}
