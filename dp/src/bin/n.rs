#[rustfmt::skip]
use std::io::{stdout, Write, BufWriter};

fn main() {
    let mut sc = InputScanOnce::new(io::stdin(), 266_666_679);
    let out = stdout();

    let mut out = BufWriter::new(out.lock());
    let n = sc.next::<usize>();
    unsafe {
        for _ in 0..n {
            write!(out, "{}\n", _popcnt64(sc.next::<u64>())).unwrap();
        }
    }
}
#[inline]
#[target_feature(enable = "popcnt")]
pub unsafe fn _popcnt64(x: u64) -> i32 {
    x.count_ones() as i32
}
use std::fmt;
use std::io::{self, Read};
use std::str::{self, FromStr};

struct InputScanOnce {
    buf: Vec<u8>, // Stores the entire input
    pos: usize,   // Should never be out of bounds
}

impl InputScanOnce {
    fn new<R: Read>(mut reader: R, estimated: usize) -> Self {
        let mut buf = Vec::with_capacity(estimated);
        let _ = io::copy(&mut reader, &mut buf).unwrap();
        if buf.last() != Some(&b'\n') {
            panic!("{}", "");
        }
        InputScanOnce { buf, pos: 0 }
    }

    #[inline]
    fn next<T: FromStr>(&mut self) -> T
    where
        T::Err: fmt::Debug,
    {
        let mut start = None;
        loop {
            match (self.buf[self.pos], start.is_some()) {
                (b' ', true) | (b'\n', true) => break,
                (_, true) | (b' ', false) | (b'\n', false) => self.pos += 1,
                (_, false) => start = Some(self.pos),
            }
        }
        let target = &self.buf[start.unwrap()..self.pos];
        unsafe { str::from_utf8_unchecked(target) }.parse().unwrap()
    }
}
