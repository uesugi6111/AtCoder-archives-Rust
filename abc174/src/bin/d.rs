#[allow(unused_imports)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(n: usize, c: String);
    let mut s: Vec<char> = c.chars().collect();

    let mut count = 0;
    let mut index_front = 0;
    let mut index_back = c.len() - 1;
    loop {
        loop {
            if index_front > n - 1 || s[index_front] == 'W' {
                break;
            }
            index_front += 1;
        }
        loop {
            if index_back == 0 || s[index_back] == 'R' {
                break;
            }
            index_back -= 1;
        }

        if index_front > n - 1 || index_back == 0 || index_front >= index_back {
            break;
        }

        //交換
        // indexが交差していた場合終了
        s[index_front] = 'R';
        s[index_back] = 'W';
        count += 1;
    }
    println!("{}", count);
}
