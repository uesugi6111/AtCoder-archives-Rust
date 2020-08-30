use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(a: usize, b: usize, k: usize);
    let a_d = divisors(a);
    let b_d = divisors(b);
    let mut vvv = Vec::new();
    for v in &a_d {
        for vv in &b_d {
            if v == vv && !vvv.contains(v) {
                vvv.push(*v);
            }
        }
    }
    vvv.sort_by_key(|w| std::cmp::Reverse(*w));
    println!("{}", vvv[k - 1]);
}

fn divisors(n: usize) -> Vec<usize> {
    let mut divisors = Vec::new();
    // n := i * x とおくと、 i が i > root(n) の時、　i はすでに ある x に探索されているから
    // i <= root(n) まで探索すればよい
    for i in 1..=(f64::sqrt(n as f64) + 1e-9) as usize {
        // i で n が割り切れた場合
        if n % i == 0 {
            // 約数リストに格納
            divisors.push(i);

            // n := i * x の x を格納。ただし x := i の時は除く
            if i != n / i {
                divisors.push(n / i);
            }
        }
    }
    divisors.sort();
    divisors
}
