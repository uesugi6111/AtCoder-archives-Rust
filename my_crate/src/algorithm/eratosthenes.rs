#[cfg(test)]
mod tests {

    use super::sieve;

    #[test]
    fn era() {
        assert_eq!(sieve(100000000).len(), 5761455);
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
