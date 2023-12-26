use std::io::prelude::*;
use std::process::Command;
use std::process::Stdio;
use std::{fs::File, path::Path};

use itertools::Itertools;

fn main() {
    compile();
    let mut max = 0;
    let mut t = (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0);
    let high_priority_width = 0;
    let work_single_width = 0;
    let work_all_width = 0;
    for final_phase_for_capital_increase in (90..91) {
        for reset_interval in (100..101).step_by(10) {
            for high_priority in 12 - high_priority_width..=12 + high_priority_width {
                for work_single in 16 - work_single_width..17 + work_single_width {
                    for work_all in 15 - work_all_width..16 + work_all_width {
                        for internal_reserves_ratio in 0..1 {
                            for final_phase_for_work in 100..=100 {
                                for final_phase_for_reset in (84..85).step_by(2) {
                                    for capital_increase_limit in 20..21 {
                                        for work_all_magnification in (18..19) {
                                            for final_phase_for_work_all in (99..100).step_by(2) {
                                                let mut sum = 0;

                                                let mut b = (0..400)
                                                    .map(|i| {
                                                        let buff = exec(
                                                            i,
                                                            final_phase_for_capital_increase,
                                                            reset_interval,
                                                            high_priority,
                                                            work_single,
                                                            work_all,
                                                            internal_reserves_ratio,
                                                            final_phase_for_work,
                                                            final_phase_for_reset,
                                                            capital_increase_limit,
                                                            work_all_magnification,
                                                            final_phase_for_work_all,
                                                        );
                                                        sum += buff;
                                                        (i, buff)
                                                    })
                                                    .collect::<Vec<_>>();
                                                let minmax =
                                                    b.iter().copied().minmax_by_key(|&i| i.1);
                                                b.sort_by_key(|x| x.1);
                                                let mid = b[b.len() / 2];
                                                let (min, maxx) = minmax.into_option().unwrap();

                                                println!(
                                                    "score: {}",
                                                    add_separator(&sum.to_string(), 3, ",")
                                                );
                                                println!(
                                "final_phase_for_capital_increase: {}, reset_interval: {}, high_priority: {}, work_single: {}, work_all: {}, internal_reserves_ratio: {}, final_phase_for_work: {}, final_phase_for_reset: {}, capital_increase_limit: {}, work_all_magnification: {}, final_phase_for_work_all: {}",
                                final_phase_for_capital_increase,
                                                    reset_interval,
                                                    high_priority,
                                                    work_single,
                                                    work_all,
                                                    internal_reserves_ratio,
                                                    final_phase_for_work,
                                                    final_phase_for_reset,
                                                    capital_increase_limit,
                                                    work_all_magnification,final_phase_for_work_all
                            );
                                                println!(
                                                    "max_score: {}, max_index: {}",
                                                    add_separator(&maxx.1.to_string(), 3, ","),
                                                    maxx.0
                                                );
                                                println!(
                                                    "mid_score: {}, mid_index: {}",
                                                    add_separator(&mid.1.to_string(), 3, ","),
                                                    mid.0
                                                );
                                                println!(
                                                    "min_score: {}, min_index: {}",
                                                    add_separator(&min.1.to_string(), 3, ","),
                                                    min.0
                                                );

                                                println!();
                                                if mid.1 >= max {
                                                    max = mid.1;
                                                    t = (
                                                        final_phase_for_capital_increase,
                                                        reset_interval,
                                                        high_priority,
                                                        work_single,
                                                        work_all,
                                                        internal_reserves_ratio,
                                                        final_phase_for_work,
                                                        final_phase_for_reset,
                                                        capital_increase_limit,
                                                        work_all_magnification,
                                                        final_phase_for_work_all,
                                                    );
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    println!("score: {}", add_separator(&max.to_string(), 3, ","));
    println!(
"final_phase_for_capital_increase: {}, reset_interval: {}, high_priority: {}, work_single: {}, work_all: {}, internal_reserves_ratio: {}, final_phase_for_work: {}, final_phase_for_reset: {}, capital_increase_limit: {}, work_all_magnification: {}",
t.0, t.1, t.2, t.3, t.4,t.5,t.6,t.7,t.8,t.9
);

    //println!("score: {}, min_index: {}", minmin.0, minmin.1);
}

fn add_separator(txt: &str, n: usize, separator: &str) -> String {
    txt.chars()
        .rev()
        .collect::<Vec<char>>()
        .chunks(n)
        .map(|cs| cs.iter().collect::<String>())
        .collect::<Vec<String>>()
        .join(separator)
        .chars()
        .rev()
        .collect::<String>()
}

fn compile() {
    Command::new("powershell")
        .args(&["/C", "cargo", "build", "--package", "ahc029", "--release"])
        .output()
        .expect("failed to execute process");
}
fn exec(
    input_file: i64,
    final_phase_for_capital_increase: i64,
    reset_interval: i64,
    high_priority: i64,
    work_single: i64,
    work_all: i64,
    internal_reserves_ratio: i64,
    final_phase_for_work: i64,
    final_phase_for_reset: i64,
    capital_increase_limit: i64,
    work_all_magnification: i64,
    final_phase_for_work_all: i64,
) -> i64 {
    let mut process = Command::new("powershell")
        .args(&[
            "/C",
            r"d:\Users\admin\download\45e6da0b06_windows\tools_windows\tester.exe",
            r"D:\src\AtCoder-archives-Rust\target\release\ahc029-a.exe",
        ])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .env(
            "FINAL_PHASE_FOR_CAPITAL_INCREASE",
            final_phase_for_capital_increase.to_string(),
        )
        .env("RESET_INTERVAL", reset_interval.to_string())
        .env("HIGH_PRIORITY", high_priority.to_string())
        .env("WORK_SINGLE", work_single.to_string())
        .env("WORK_ALL", work_all.to_string())
        .env(
            "INTERNAL_RESERVES_RATIO",
            internal_reserves_ratio.to_string(),
        )
        .env("FINAL_PHASE_FOR_WORK", final_phase_for_work.to_string())
        .env("FINAL_PHASE_FOR_RESET", final_phase_for_reset.to_string())
        .env("CAPITAL_INCREASE_LIMIT", capital_increase_limit.to_string())
        .env("WORK_ALL_MAGNIFICATION", work_all_magnification.to_string())
        .env(
            "FINAL_PHASE_FOR_WORK_ALL",
            final_phase_for_work_all.to_string(),
        )
        .spawn()
        .expect("failed to execute process");

    let mut stdin = process.stdin.take().expect("Failed to open stdin");
    let p = format!(
        r"d:\Users\admin\download\45e6da0b06_windows\tools_windows\in\{:0>4}.txt",
        input_file
    );
    let path = Path::new(&p);
    let display = path.display();
    let mut file = match File::open(&path) {
        // The `description` method of `io::Error` returns a string that
        // describes the error
        // `io::Error`の`description`メソッドはエラーを説明する文字列を返す。
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };
    let mut s: String = String::new();
    let _ = file.read_to_string(&mut s).is_ok();
    std::thread::spawn(move || {
        stdin
            .write_all(s.as_bytes())
            .expect("Failed to write to stdin");
    });
    let mut s = String::new();
    match process.stdout.unwrap().read_to_string(&mut s) {
        Err(why) => panic!("couldn't read stdout: {}", why),
        Ok(_) => {}
    }

    let mut stderr = String::new();
    match process.stderr.unwrap().read_to_string(&mut stderr) {
        Err(why) => panic!("couldn't read  stderr: {}", why),
        Ok(_) => {}
    }

    let mut splited_stderr = stderr.split_ascii_whitespace();
    splited_stderr.next();
    splited_stderr.next();
    splited_stderr.next().unwrap().parse::<i64>().unwrap()
}
