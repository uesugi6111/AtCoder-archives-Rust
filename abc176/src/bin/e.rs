#[allow(unused_imports)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(h: usize, w: usize, m: usize, hw: [[usize; 2]; m]);

    let mut w_sum = vec![0usize; h];
    let mut h_sum = vec![0usize; w];

    let mut hw_map = std::collections::HashMap::<String, usize>::new();

    for v in hw {
        w_sum[(v[0] - 1) as usize] += 1;
        h_sum[(v[1] - 1) as usize] += 1;
        let s = format!(
            "{}{}{}",
            (v[0] - 1).to_string(),
            "_".to_string(),
            (v[1] - 1).to_string()
        );

        if hw_map.contains_key(&s) {
            let a = hw_map.get(&s).unwrap();
            hw_map.insert(s, a + 1);
        } else {
            hw_map.insert(s, 1usize);
        }
    }

    let mut w_max = 0;
    let mut w_max_index = Vec::new();
    for i in 0..w_sum.len() {
        if w_max == w_sum[i] {
            w_max_index.push(i);
        } else if w_max < w_sum[i] {
            w_max_index = Vec::new();
            w_max_index.push(i);
            w_max = w_sum[i];
        }
    }

    let mut h_max = 0;
    let mut h_max_index = Vec::new();
    for i in 0..h_sum.len() {
        if h_max == h_sum[i] {
            h_max_index.push(i);
        } else if h_max < h_sum[i] {
            h_max_index = Vec::new();
            h_max_index.push(i);
            h_max = h_sum[i];
        }
    }
    let mut mainasu = 1;
    for v in h_max_index {
        for vw in &w_max_index {
            let s = format!("{}{}{}", vw.to_string(), "_".to_string(), v.to_string());
            if !hw_map.contains_key(&s) {
                mainasu = 0;
                break;
            }
        }
        if mainasu == 0 {
            break;
        }
    }

    println!("{}", w_max + h_max - mainasu);
}
