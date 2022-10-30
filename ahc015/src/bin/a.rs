use std::io::{self, BufReader};

use proconio::{input, source::line::LineSource};

pub const N: usize = 10;
pub const M: usize = 3;

fn main() {
    let mut stdin = LineSource::new(BufReader::new(io::stdin()));
    macro_rules! input(($($tt:tt)*) => (proconio::input!(from &mut stdin, $($tt)*)));

    input!(fs: [usize; 100]);

    let mut count = std::collections::HashMap::new();
    for i in 0..100 {
        *count.entry(fs[i]).or_insert(0) += 1;
    }
    let mut v = count.iter().map(|(a, b)| (*a, *b)).collect::<Vec<_>>();
    v.sort_by_key(|x| -x.1);

    let mut ps = vec![];

    let mut d = vec![];
    for i in 0..98 {
        input!(x: usize);
        ps.push(x);

        let dd = if fs[i + 1] == v[0].0 {
            'L'
        } else if fs[i + 1] == v[1].0 {
            'B'
        } else {
            'F'
        };
        println!("{}", dd);
        d.push(dd);
    }
    input!(x: usize);
    ps.push(x);
    ps.push(1);

    let inp = Input { fs, ps };
    dbg!(inp.fs.len(), inp.ps.len(), d.len());

    let mut vvvvv = vec![];
    dbg!(&vvvvv);
    vvvvv.push({
        let c = 'F';
        let mut ddd = d.clone();
        ddd.push(c);

        (c, compute_score(&inp, &ddd))
    });
    vvvvv.push({
        let c = 'B';
        let mut ddd = d.clone();
        ddd.push(c);

        (c, compute_score(&inp, &ddd))
    });

    vvvvv.push({
        let c = 'L';
        let mut ddd = d.clone();
        ddd.push(c);

        (c, compute_score(&inp, &ddd))
    });
    vvvvv.push({
        let c = 'R';
        let mut ddd = d.clone();
        ddd.push(c);

        (c, compute_score(&inp, &ddd))
    });

    vvvvv.sort_by_key(|x| -x.1 .0);

    println!("{}", vvvvv[0].0);
    input!(_: usize);
    println!();
}

#[derive(Clone, Debug)]
pub struct State {
    pub fs: Vec<usize>,
    pub ps: Vec<usize>,
    pub board: Vec<Vec<usize>>,
    pub t: usize,
    pub last: (usize, usize),
}

#[macro_export]
macro_rules! mat {
	($($e:expr),*) => { Vec::from(vec![$($e),*]) };
	($($e:expr,)*) => { Vec::from(vec![$($e),*]) };
	($e:expr; $d:expr) => { Vec::from(vec![$e; $d]) };
	($e:expr; $d:expr $(; $ds:expr)+) => { Vec::from(vec![mat![$e $(; $ds)*]; $d]) };
}
#[derive(Clone, Debug)]
pub struct Input {
    pub fs: Vec<usize>,
    pub ps: Vec<usize>,
}
impl State {
    pub fn new(input: &Input) -> Self {
        let mut board = mat![0; N; N];
        let last = ((input.ps[0] - 1) / N, (input.ps[0] - 1) % N);
        board[last.0][last.1] = input.fs[0];
        Self {
            fs: input.fs.clone(),
            ps: input.ps.clone(),
            board,
            t: 0,
            last,
        }
    }
    pub fn apply_move(&mut self, dir: char) -> Result<(), String> {
        match dir {
            'L' => {
                for i in 0..N {
                    let mut k = 0;
                    for j in 0..N {
                        if self.board[i][j] != 0 {
                            self.board[i][k] = self.board[i][j];
                            if k != j {
                                self.board[i][j] = 0;
                            }
                            k += 1;
                        }
                    }
                }
            }
            'R' => {
                for i in 0..N {
                    let mut k = N - 1;
                    for j in (0..N).rev() {
                        if self.board[i][j] != 0 {
                            self.board[i][k] = self.board[i][j];
                            if k != j {
                                self.board[i][j] = 0;
                            }
                            k -= 1;
                        }
                    }
                }
            }
            'F' => {
                for j in 0..N {
                    let mut k = 0;
                    for i in 0..N {
                        if self.board[i][j] != 0 {
                            self.board[k][j] = self.board[i][j];
                            if k != i {
                                self.board[i][j] = 0;
                            }
                            k += 1;
                        }
                    }
                }
            }
            'B' => {
                for j in 0..N {
                    let mut k = N - 1;
                    for i in (0..N).rev() {
                        if self.board[i][j] != 0 {
                            self.board[k][j] = self.board[i][j];
                            if k != i {
                                self.board[i][j] = 0;
                            }
                            k -= 1;
                        }
                    }
                }
            }
            _ => {
                return Err(format!("Illegal output: {}", dir));
            }
        }
        self.t += 1;
        let mut p = 0;
        for i in 0..N {
            for j in 0..N {
                if self.board[i][j] == 0 {
                    p += 1;
                    if p == self.ps[self.t] {
                        self.board[i][j] = self.fs[self.t];
                        self.last = (i, j);
                    }
                }
            }
        }
        Ok(())
    }
}

pub const DIJ: [(usize, usize); 4] = [(1, 0), (0, 1), (!0, 0), (0, !0)];

pub fn compute_score(input: &Input, out: &[char]) -> (i64, String, State) {
    let mut state = State::new(input);
    for t in 0..out.len().min(N * N - 1) {
        if let Err(err) = state.apply_move(out[t]) {
            return (0, format!("{} (turn: {})", err, t), state);
        }
    }
    let mut visited = mat![false; N; N];
    let mut num = 0;
    for i in 0..N {
        for j in 0..N {
            if !visited[i][j] && state.board[i][j] != 0 {
                visited[i][j] = true;
                let c = state.board[i][j];
                let mut size = 1;
                let mut stack = vec![(i, j)];
                while let Some((i, j)) = stack.pop() {
                    for &(di, dj) in &DIJ {
                        let i2 = i + di;
                        let j2 = j + dj;
                        if i2 < N && j2 < N && !visited[i2][j2] && state.board[i2][j2] == c {
                            visited[i2][j2] = true;
                            stack.push((i2, j2));
                            size += 1;
                        }
                    }
                }
                num += size * size;
            }
        }
    }
    let mut d = vec![0; M + 1];
    for &f in &input.fs {
        d[f] += 1;
    }
    let score =
        (1e6 * num as f64 / d[1..].iter().map(|d| d * d).sum::<i32>() as f64).round() as i64;
    (score, String::new(), state)
}
