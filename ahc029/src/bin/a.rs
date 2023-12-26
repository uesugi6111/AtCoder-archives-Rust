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
        .unwrap_or("81".to_owned())
        .parse()
        .unwrap()
});
pub static FINAL_PHASE_FOR_WORK: Lazy<usize> = Lazy::new(|| {
    env::var("FINAL_PHASE_FOR_WORK")
        .unwrap_or("100".to_owned())
        .parse()
        .unwrap()
});
pub static FINAL_PHASE_FOR_RESET: Lazy<usize> = Lazy::new(|| {
    env::var("FINAL_PHASE_FOR_RESET")
        .unwrap_or("84".to_owned())
        .parse()
        .unwrap()
});
pub static RESET_INTERVAL: Lazy<usize> = Lazy::new(|| {
    env::var("RESET_INTERVAL")
        .unwrap_or("100".to_owned())
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
        .unwrap_or("16".to_owned())
        .parse()
        .unwrap()
});
pub static WORK_ALL: Lazy<i64> = Lazy::new(|| {
    env::var("WORK_ALL")
        .unwrap_or("15".to_owned())
        .parse()
        .unwrap()
});

pub static INTERNAL_RESERVES_RATIO: Lazy<i64> = Lazy::new(|| {
    env::var("INTERNAL_RESERVES_RATIO")
        .unwrap_or("0".to_owned())
        .parse()
        .unwrap()
});

pub static WORK_ALL_MAGNIFICATION: Lazy<i64> = Lazy::new(|| {
    env::var("WORK_ALL_MAGNIFICATION")
        .unwrap_or("18".to_owned())
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

    let mut money = 0;
    let mut capital_increase_count = tw.iter().filter(|x| x.0 == 4).count() as u32;

    let mut bought_capital_increase_count =0;
    let mut last_reset = 0;
    //let mut hakishitai = None;

    for i in 0..t {
        let mut buff = hv.iter().enumerate().collect::<Vec<_>>();
        buff.sort_by_key(|x| (x.1 .0) - (x.1 .1));
        let y = if let Some(y) = buff
            .iter()
            .find(|x| x.1 .0 < *HIGH_PRIORITY * 2i64.pow(capital_increase_count))
        {
            y.clone()
        } else {
            buff[0].clone()
        };
        let use_index = {
            let reset = tw.iter().find_position(|x| x.0 == 2);
            if reset.is_some() {
                last_reset = i;
                reset.unwrap().0
            } else {
                let a = tw.iter().find_position(|x| x.0 == 4||x.0==3).map(|x| x.0);

                let use_index = a.unwrap_or(
                    tw.iter()
                        .enumerate()
                        .filter(|x| {
                            (x.1 .0 == 1)
                                || (x.1 .0 == 0
                                    && (((x.1 .1 <= y.1 .0&&i * 100 < t * 95)|| i * 100 >= t * 95)
                                        || x.1 .1 <= 2i64.pow(capital_increase_count)))
                        })
                        .max_by_key(|x| x.1)
                        .map(|x| x.0)
                        .unwrap_or(0),
                );
                use_index
            }
        };

        let target = if tw[use_index].0 == 0 {
            if i * 100 < t * 85 {
            let mut yy = y.0;
            for _ in 0..m {
                if hv[yy % m].0
                    > ((t - 1) - i - 1) as i64 * 2i64.pow(capital_increase_count) * *WORK_ALL_MAGNIFICATION/10+ tw[use_index].1 
                {
                    yy += 1;
                } else {
                    break;
                }
            }
            yy % m}else if let Some(x)=buff.iter().find(|x|x.1.0<=((t - 1) - i ) as i64 * 2i64.pow(capital_increase_count) + tw[use_index].1 )
             {
x.0
                
          
            }else{      let mut buff = hv.iter().enumerate().collect::<Vec<_>>();
        buff.sort_by_key(|x| (x.1 .0) );



                buff[0].0
                
            }
        // {y.0}
            
        } else if tw[use_index].0 == 2 {
            buff[buff.len() - 1].0
        } else {
            0
        };

        println!("{} {}", use_index, target);

        input!(input_hv: [(i64, i64); m]);
        hv = input_hv;
        input!(input_money: i64);
        money = input_money;

        input!(twp: [(usize, i64, i64); k]);

        let get_index = {
            let reset = twp.iter().find_position(|x| x.0 == 2
                ||x.0==3
            ); 
            if reset.is_some()
                && reset.unwrap().1 .2 < money
                && ((reset.unwrap().1 .0 ==2 &&i - last_reset > *RESET_INTERVAL                && i * 100 < t * *FINAL_PHASE_FOR_RESET) 
                || (i * 100 < t * 97&&
                    reset.unwrap().1 .0 ==3&& hv.iter().find(|y|y.0<100 * 2i64.pow(capital_increase_count)  ).is_none()))
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
                } else {
                    twp.iter()
                        .find_position(|x| {
                            x.0 == 4
                                && x.2*2 <= money
                                && capital_increase_count < *CAPITAL_INCREASE_LIMIT
                        })
                        .map(|x| x.0)
                };

                let get_index = b.unwrap_or(if i * 100 < t * *FINAL_PHASE_FOR_WORK {
                    twp.iter()
                        .enumerate()
                        .rev()
                        .find_position(|x| {
                            x.1 .2
                                <= money
                                    - 
                                   // if i * 100 < t * *FINAL_PHASE_FOR_CAPITAL_INCREASE {
                                        get_internal_reserves(capital_increase_count)
                                 //   } else {
                                 //       0
                                 //   }
                                && ((x.1 .0 == 1
                                    && x.1 .2 * *WORK_ALL
                                        < hv.iter()
                                            .map(|y| {
                                                x.1 .1 //   ((y.0 * *WORK_ALL_MAGNIFICATION) / 10).min(x.1 .1)
                                              // ((y.0 * *WORK_ALL_MAGNIFICATION) / 10).min(x.1 .1)
                                            })
                                            .sum::<i64>()
                                            * 10)
                                    || (x.1 .0 == 0 && x.1 .2 * *WORK_SINGLE < x.1 .1 * 10
                                    &&hv.iter().find(|y|y.0 <((t - 1) - i - 1) as i64 * 2i64.pow(capital_increase_count) * *WORK_ALL_MAGNIFICATION/10+ x.1.1 ).is_some()))
                        })
                        .map(|x| x.1 .0)
                        .unwrap_or(0)
                } else {
                    0
                });
                if twp[get_index].0 == 4 {
                    capital_increase_count += 1;
                    bought_capital_increase_count==1;
                }
                get_index
            }
        };
        println!("{}", get_index);
        tw[use_index] = (twp[get_index].0, twp[get_index].1);
    }
}

fn get_internal_reserves(capital_increase_count: u32) -> i64 {
    if capital_increase_count < *CAPITAL_INCREASE_LIMIT {
        *INTERNAL_RESERVES_RATIO * 2i64.pow(capital_increase_count)
    } else {
        0
    }
}
