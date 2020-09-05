use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(x: usize);
    if x == 1 {
        println!("1");
        return;
    }

    let mut b_v = vec![false; 1001];

    for v in 1..x {
        for i in 2..1000f32.sqrt() as u32 {
            let buff = v.pow(i) as usize;
            if buff > 1000 {
                break;
            }
            b_v[buff] = true;
        }
    }

    for i in (0..x + 1).rev() {
        if b_v[i] {
            println!("{}", i);
            return;
        }
    }
}
