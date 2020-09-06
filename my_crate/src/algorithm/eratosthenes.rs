#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn era() {
        assert_eq!(sieve_liner(10_000_000).len(), 5761455);
    }
}

/// エラトステネスの篩
/// 少し早い
/// 引数までの素数を返す
pub fn sieve(n: usize) -> Vec<usize> {
    let mut ps: Vec<usize> = vec![2];
    let mut xs: Vec<bool> = vec![true; n / 2];

    let mut x = 3;
    while x * x <= n {
        let mut y = (x - 3) / 2;
        if xs[y] {
            ps.push(x);
            y += x;
            for v in xs.iter_mut().skip(y).step_by(x) {
                *v = false;
            }
        }
        x += 2;
    }
    for v in (x..n + 1).step_by(2).filter(|x| xs[(*x - 3) / 2] == true) {
        ps.push(v);
    }
    ps
}

///素因数列挙
pub fn sieve_liner(n: usize) -> Vec<usize> {
    let size = (n as f64 / (n as f64).ln() * 1.105) as usize;

    let mut primes = Vec::with_capacity(size);
    let mut d = vec![0usize; n + 1];
    for i in 2..n + 1 {
        if d[i] == 0 {
            primes.push(i);
            d[i] = i;
        }
        for p in &primes {
            if p * i > n {
                break;
            }
            d[*p * i] = *p;
        }
    }
    d
}

#[test]
fn aaa() {
    let n = 1000usize;
    let size = (n as f64 / (n as f64).ln() * 1.105) as usize;

    println!("{}", size);
}

//y = sum[i = 2..x/log(x)] x/log(x)
