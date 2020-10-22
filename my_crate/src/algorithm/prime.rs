use std::collections::HashMap;

///試割
pub fn trial_division(mut n: usize) -> HashMap<usize, usize> {
    let mut primes = HashMap::new();
    let mut i = 2;

    while i * i <= n {
        while n % i == 0 {
            n /= i;
            primes.entry(i).and_modify(|e| *e += 1).or_insert(1);
        }
        i += 1;
    }
    if n > 1 {
        primes.entry(n).and_modify(|e| *e += 1).or_insert(1);
    }
    primes
}

#[test]
fn test_trial_division() {
    assert!(trial_division(25).contains_key(&5));
    assert!(trial_division(25).get(&5).unwrap() == &2);
}
