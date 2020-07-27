#[allow(unused_imports)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(n: usize, p: [usize; n], q: [usize; n]);
    let permutations = make_permutation(n);

    let mut p_num: i32 = 0;
    let mut q_num: i32 = 0;

    for h in 0..permutations.len() {
        let mut is_p = true;
        for i in 0..n {
            if p[i] != permutations[h][i] + 1 {
                is_p = false;
                break;
            }
        }
        if is_p {
            p_num = h as i32;
        }
        let mut is_q = true;
        for i in 0..n {
            if q[i] != permutations[h][i] + 1 {
                is_q = false;
                break;
            }
        }
        if is_q {
            q_num = h as i32;
        }
    }

    println!("{}", num::abs(p_num - q_num));
}

fn make_permutation(n: usize) -> Vec<Vec<usize>> {
    let mut vecs: Vec<Vec<usize>> = vec![Vec::new(); factorial(n)];
    let nums: Vec<usize> = (0..n).collect();
    let indexes: Vec<usize> = (0..factorial(n)).collect();
    push_recusive(nums, indexes, &mut vecs);
    vecs
}

fn push_recusive<T: Clone>(
    nums: Vec<T>,
    indexes: Vec<usize>,
    vecs: &mut Vec<Vec<T>>,
) -> &mut Vec<Vec<T>> {
    if nums.len() == 0 {
        return vecs;
    }
    let block_size = factorial(nums.len() - 1);
    for (block_index, num) in nums.iter().enumerate() {
        for inner_index in 0..block_size {
            let index = indexes[block_size * block_index + inner_index];
            vecs[index].push(num.clone());
        }
        let new_nums = {
            let mut tmp = nums.clone();
            tmp.remove(block_index);
            tmp
        };
        let new_indexes: Vec<usize> = {
            let slice = &indexes[(block_size * block_index)..(block_size * (block_index + 1))];
            slice.to_vec()
        };
        push_recusive(new_nums, new_indexes, vecs);
    }
    vecs
}

fn factorial(i: usize) -> usize {
    if i <= 1 {
        1
    } else {
        (2..=i).fold(1, |acc, x| acc * x)
    }
}
