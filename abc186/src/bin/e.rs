use num::integer::gcd;
use num::integer::Integer;
use proconio::input;

fn main() {
    input!(t: usize);

    for _ in 0..t {
        input!(mut n: i64, s: i64, mut k: i64);

        let g = gcd(n, gcd(s, k));
        let ng = n / g;
        let kg = k / g;
        let sg = s / g;

        if gcd(ng, kg) != 1 {
            println!("-1");
            continue;
        }

        let e = kg.extended_gcd(&ng);

        println!("{}", if e.x > 0 { e.x } else { e.x + ng } * (ng - sg) % ng);
    }
}
