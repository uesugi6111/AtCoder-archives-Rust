use std::{
    collections::VecDeque,
    io::{self, BufReader},
};

use itertools::Itertools;
use once_cell::sync::Lazy;
use proconio::{input, source::line::LineSource};
use std::env;

pub static FINAL_PHASE_FOR_CAPITAL_INCREASE: Lazy<usize> = Lazy::new(|| {
    env::var("FINAL_PHASE_FOR_CAPITAL_INCREASE")
        .unwrap_or("80".to_owned())
        .parse()
        .unwrap()
});
pub static FINAL_PHASE_FOR_WORK: Lazy<usize> = Lazy::new(|| {
    env::var("FINAL_PHASE_FOR_WORK")
        .unwrap_or("98".to_owned())
        .parse()
        .unwrap()
});
pub static FINAL_PHASE_FOR_WORK_ALL: Lazy<usize> = Lazy::new(|| {
    env::var("FINAL_PHASE_FOR_WORK_ALL")
        .unwrap_or("98".to_owned())
        .parse()
        .unwrap()
});
pub static FINAL_PHASE_FOR_RESET: Lazy<usize> = Lazy::new(|| {
    env::var("FINAL_PHASE_FOR_RESET")
        .unwrap_or("95".to_owned())
        .parse()
        .unwrap()
});
pub static RESET_INTERVAL: Lazy<usize> = Lazy::new(|| {
    env::var("RESET_INTERVAL")
        .unwrap_or("110".to_owned())
        .parse()
        .unwrap()
});
pub static HIGH_PRIORITY: Lazy<i64> = Lazy::new(|| {
    env::var("HIGH_PRIORITY")
        .unwrap_or("12".to_owned())
        .parse()
        .unwrap()
});
pub static CAPITAL_INCREASE_LIMIT: Lazy<u32> = Lazy::new(|| {
    env::var("CAPITAL_INCREASE_LIMIT")
        .unwrap_or("20".to_owned())
        .parse()
        .unwrap()
});
pub static WORK_SINGLE: Lazy<i64> = Lazy::new(|| {
    env::var("WORK_SINGLE")
        .unwrap_or("15".to_owned())
        .parse()
        .unwrap()
});
pub static WORK_ALL: Lazy<i64> = Lazy::new(|| {
    env::var("WORK_ALL")
        .unwrap_or("12".to_owned())
        .parse()
        .unwrap()
});

pub static INTERNAL_RESERVES_RATIO: Lazy<i64> = Lazy::new(|| {
    env::var("INTERNAL_RESERVES_RATIO")
        .unwrap_or("11".to_owned())
        .parse()
        .unwrap()
});

pub static WORK_ALL_MAGNIFICATION: Lazy<i64> = Lazy::new(|| {
    env::var("WORK_ALL_MAGNIFICATION")
        .unwrap_or("0".to_owned())
        .parse()
        .unwrap()
});

fn main() {
    let mut stdin = LineSource::new(BufReader::new(io::stdin()));
    macro_rules! input(($($tt:tt)*) => (proconio::input!(from &mut stdin, $($tt)*)));
    input!(n: usize, m: usize, k: usize, t: usize);
    input!(input_tw: [(usize, i64); n], input_hv: [(i64, i64); m]);

    let mut hv = input_hv;

    let mut tw = input_tw.iter().copied().collect::<VecDeque<_>>();

    let mut capital_increase_count = tw.iter().filter(|x| x.0 == 4).count() as u32;

    let mut last_reset = 0;

    for i in 0..t {
        let mut buff = hv.iter().enumerate().collect::<Vec<_>>();
        buff.sort_by_key(|x| (x.1 .0) - (x.1 .1));
        let mut y = if let Some(y) = buff
            .iter()
            .find(|x| x.1 .0 < *HIGH_PRIORITY * 2i64.pow(capital_increase_count))
        {
            y.clone()
        // } else if let Some(y) = buff
        //     .iter()
        //     .find(|x| x.1 .0 < ((t - 1) - i) as i64 * 2i64.pow(capital_increase_count))
        // {
        //     y.clone()
        } else {
            buff[0].clone()
        };

        let use_index = {
            let mut buff_y = buff[buff.len() - 1].clone();
            let mut r = if let Some(reset) = tw.iter().find_position(|x| x.0 == 2) {
                if let Some(murimuri) = buff
                    .iter()
                    .find(|x| x.1 .0 > (t - i) as i64 * 2i64.pow(capital_increase_count))
                {
                    //last_reset = 0;
                    buff_y = murimuri.clone();
                    Some(reset.0)
                } else if let Some(tyotto_muri) = buff.iter().max_by_key(|(_, x)| x.0 - x.1) {
                    //  .find(|(_, x)| x.0 - x.1 < 0 * 2i64.pow(capital_increase_count))
                    if tyotto_muri.1 .0 - tyotto_muri.1 .1 > 25 * 2i64.pow(capital_increase_count) {
                        buff_y = tyotto_muri.clone();
                        Some(reset.0)
                    } else {
                        None
                    }
                } else {
                    None
                }
            } else {
                None
            };
            if r.is_some() && *RESET_INTERVAL < i - last_reset {
                last_reset = i;
                y = buff_y;
            } else {
                r = None;
            }
            if let Some(rr) = r {
                rr
            } else {
                let a = tw
                    .iter()
                    .find_position(|x| (x.0 == 4) || x.0 == 3)
                    .map(|x| x.0);

                let use_index = a.unwrap_or(
                    tw.iter()
                        .enumerate()
                        .filter(|x| (x.1 .0 == 1) || (x.1 .0 == 0))
                        .max_by_key(|x| x.1)
                        .map(|x| x.0)
                        .unwrap_or(0),
                );
                use_index
            }
        };

        let target = if tw[use_index].0 == 0 {
            let mut yy = y.0;
            for _ in 0..m {
                if hv[yy % m].0
                    > ((t - 1) - i - 1) as i64 * 2i64.pow(capital_increase_count) + tw[use_index].1
                {
                    yy += 1;
                } else {
                    break;
                }
            }
            yy % m
        } else if tw[use_index].0 == 2 {
            y.0
        } else {
            0
        };

        println!("{} {}", use_index, target);

        input!(input_hv: [(i64, i64); m]);
        hv = input_hv;
        input!(money: i64);

        input!(twp: [(usize, i64, i64); k]);

        let mut buff = hv.iter().enumerate().collect::<Vec<_>>();
        buff.sort_by_key(|x| (x.1 .0) - (x.1 .1));

        let get_index = {
            let reset = twp.iter().find_position(|x| x.0 == 2);
            if reset.is_some()
                && reset.unwrap().1 .2 < money
                && reset.unwrap().1 .2 < 7 * 2i64.pow(capital_increase_count)
                && tw.iter().find(|x| x.0 == 2).is_none()
                && i % 2 == 0
            {
                reset.unwrap().0
            } else {
                let b = if i * 100 < t * *FINAL_PHASE_FOR_CAPITAL_INCREASE {
                    twp.iter()
                        .find_position(|x| {
                            x.0 == 4
                                && x.2 <= money
                                && capital_increase_count < *CAPITAL_INCREASE_LIMIT
                        })
                        .map(|x| x.0)
                } else if i * 100 <= t * *FINAL_PHASE_FOR_RESET {
                    twp.iter()
                        .find_position(|x| {
                            x.0 == 4
                                && x.2 * *INTERNAL_RESERVES_RATIO <= money * 10
                                && capital_increase_count < *CAPITAL_INCREASE_LIMIT
                        })
                        .map(|x| x.0)
                } else {
                    None
                };

                let get_index = b.unwrap_or(if i * 100 < t * *FINAL_PHASE_FOR_WORK {
                    twp.iter()
                        .enumerate()
                        .rev()
                        .find_position(|x| {
                            x.1 .2 <= money - get_internal_reserves(capital_increase_count)
                                && ((x.1 .0 == 1
                                    && x.1 .2 * *WORK_ALL
                                        < hv.iter().map(|y| (y.0).min(x.1 .1)).sum::<i64>() * 10
                                    && i * 100 < t * *FINAL_PHASE_FOR_WORK_ALL)
                                    || (x.1 .0 == 0 && x.1 .2 * *WORK_SINGLE < x.1 .1 * 10))
                        })
                        .map(|x| x.1 .0)
                        .unwrap_or(0)
                } else {
                    twp.iter()
                        .enumerate()
                        .rev()
                        .find_position(|(_, x)| {
                            x.0 == 0
                                && x.2 <= money - get_internal_reserves(capital_increase_count)
                                && buff[0].1 .0 <= x.1
                                && (x.2 <= buff[0].1 .1)
                        })
                        .map(|x| x.1 .0)
                        .unwrap_or(0)
                });
                if twp[get_index].0 == 4 {
                    capital_increase_count += 1;
                }
                get_index
            }
        };
        println!("{}", if i == t - 1 { 0 } else { get_index });
        tw[use_index] = (twp[get_index].0, twp[get_index].1);
    }
}

fn get_internal_reserves(capital_increase_count: u32) -> i64 {
    if capital_increase_count < *CAPITAL_INCREASE_LIMIT {
        0
    } else {
        0
    }
}
