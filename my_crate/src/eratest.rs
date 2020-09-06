pub struct Eratosthenes {
    flags_: Vec<u8>,
}
impl Eratosthenes {
    const K_MASK: [[u8; 8]; 8] = [
        [0xfe, 0xfd, 0xfb, 0xf7, 0xef, 0xdf, 0xbf, 0x7f],
        [0xfd, 0xdf, 0xef, 0xfe, 0x7f, 0xf7, 0xfb, 0xbf],
        [0xfb, 0xef, 0xfe, 0xbf, 0xfd, 0x7f, 0xf7, 0xdf],
        [0xf7, 0xfe, 0xbf, 0xdf, 0xfb, 0xfd, 0x7f, 0xef],
        [0xef, 0x7f, 0xfd, 0xfb, 0xdf, 0xbf, 0xfe, 0xf7],
        [0xdf, 0xf7, 0x7f, 0xfd, 0xbf, 0xfe, 0xef, 0xfb],
        [0xbf, 0xfb, 0xf7, 0x7f, 0xfe, 0xef, 0xdf, 0xfd],
        [0x7f, 0xbf, 0xdf, 0xef, 0xf7, 0xfb, 0xfd, 0xfe],
    ];

    const C0: [[usize; 8]; 8] = [
        [0, 0, 0, 0, 0, 0, 0, 1],
        [1, 1, 1, 0, 1, 1, 1, 1],
        [2, 2, 0, 2, 0, 2, 2, 1],
        [3, 1, 1, 2, 1, 1, 3, 1],
        [3, 3, 1, 2, 1, 3, 3, 1],
        [4, 2, 2, 2, 2, 2, 4, 1],
        [5, 3, 1, 4, 1, 3, 5, 1],
        [6, 4, 2, 4, 2, 4, 6, 1],
    ];
    const K_MOD_30: [usize; 8] = [1, 7, 11, 13, 17, 19, 23, 29];
    const C1: [usize; 8] = [6, 4, 2, 4, 2, 4, 6, 2];
    pub fn new() -> Self {
        Eratosthenes {
            flags_: Vec::<u8>::new(),
        }
    }

    fn bit_to_index(b: u8) -> usize {
        b.trailing_zeros() as usize
    }

    pub fn generate(&mut self, x: usize) -> &Self {
        if x > 10000000000 {
            return self;
        }
        let size = x / 30 + if x % 30 != 0 { 1 } else { 0 };
        self.flags_.clear();
        self.flags_.resize(size, 0xff);

        self.flags_[0] = 0xfe;

        let r = x % 30;
        if r != 0 {
            if r <= 1 {
                self.flags_[(size - 1) as usize] = 0x0;
            } else if r <= 7 {
                self.flags_[(size - 1) as usize] = 0x1;
            } else if r <= 11 {
                self.flags_[(size - 1) as usize] = 0x3;
            } else if r <= 13 {
                self.flags_[(size - 1) as usize] = 0x7;
            } else if r <= 17 {
                self.flags_[(size - 1) as usize] = 0xf;
            } else if r <= 19 {
                self.flags_[(size - 1) as usize] = 0x1f;
            } else if r <= 23 {
                self.flags_[(size - 1) as usize] = 0x3f;
            } else if r <= 29 {
                self.flags_[(size - 1) as usize] = 0x7f;
            }
        }

        let quart_x = ((x as f64).sqrt().ceil() + 0.1) as usize;
        let quart_xi = quart_x / 30 + 1;

        self.flags_[0] = 0xfe;
        for i in 0..quart_xi {
            let mut flags: u8 = self.flags_[i];

            while flags != 0 {
                let lsb: u8 = flags & flags.wrapping_neg();
                let ibit = Eratosthenes::bit_to_index(lsb);

                let m = Eratosthenes::K_MOD_30[ibit];
                let pm = 30 * i + 2 * m;
                let mut k = ibit;
                let mut j = i * pm + (m * m) / 30;

                while j < self.flags_.len() {
                    self.flags_[j] &= Eratosthenes::K_MASK[ibit][k];

                    j += i * Eratosthenes::C1[k] + Eratosthenes::C0[ibit][k];
                    k = (k + 1) & 7;
                }
                flags &= flags - 1
            }
        }
        self
    }

    pub fn count(&mut self) -> i64 {
        if self.flags_.is_empty() {
            return -1;
        }

        let mut ret = 3; // count 2, 3, 5
        for f in &self.flags_ {
            ret += f.count_ones() as i64;
        }
        ret
    }
    pub fn primes(&self) -> Vec<usize> {
        let mut ret = Vec::new();
        ret.push(2);
        ret.push(3);
        ret.push(5);
        for (i, f) in self.flags_.iter().enumerate() {
            for (ii, m) in Eratosthenes::K_MOD_30.iter().enumerate() {
                if (*f & (1 << ii)) != 0 {
                    ret.push(30 * i + *m);
                }
            }
        }
        ret
    }
}

#[test]
fn test_ok() {
    let n = 10000;
    let era = Eratosthenes::new().generate(n).primes();

    println!("{}", era.len());

    let ss = sieve(n);
    println!("{}", ss.len());
}

#[test]
fn bit_to_index_test() {
    assert_eq!(Eratosthenes::bit_to_index(1 << 0), 0);
    assert_eq!(Eratosthenes::bit_to_index(1 << 1), 1);
    assert_eq!(Eratosthenes::bit_to_index(1 << 2), 2);
    assert_eq!(Eratosthenes::bit_to_index(1 << 3), 3);
    assert_eq!(Eratosthenes::bit_to_index(1 << 4), 4);
    assert_eq!(Eratosthenes::bit_to_index(1 << 5), 5);
    assert_eq!(Eratosthenes::bit_to_index(1 << 6), 6);
    assert_eq!(Eratosthenes::bit_to_index(1 << 7), 7);
}
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
