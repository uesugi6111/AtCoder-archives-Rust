#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
fn main() {
    input!(n: usize);

    let mut list = vec![];

    for _ in 0..n {
        input!(a: usize, xy: [(Usize1, usize); a]);

        let mut map = std::collections::HashMap::new();
        for x in xy {
            map.insert(x.0, x.1);
        }
        list.push(map);
    }
    let mut max = 0;
    for i in 0..(1 << n) as usize {
        let mut iiiiiiiii_true = true;
        for j in 0..n {
            let shojiki = if (1 << j) & i != 0 { 1 } else { 0 };
            let mut is_true = true;
            for (kk, k) in list.iter().enumerate() {
                if (1 << kk) & i == 0 {
                    continue;
                }

                let k_get = k.get(&j);
                if let Some(z) = k_get {
                    if *z != shojiki {
                        is_true = false;
                        break;
                    }
                }
            }
            if !is_true {
                iiiiiiiii_true = false;
                break;
            }
        }
        if iiiiiiiii_true {
            let i_pop = i.count_ones();
            max = std::cmp::max(max, i_pop as usize);
        }
    }

    println!("{}", max);
}
