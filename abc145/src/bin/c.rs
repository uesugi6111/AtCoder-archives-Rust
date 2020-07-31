#[allow(unused_imports)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(n: usize, xy: [[isize; 2]; n]);

    let perm = make_permutation(n);
    let mut count = 0;
    let mut sum: f64 = 0.0;
    for v in perm {
        for i in 0..v.len() - 1 {
            sum += (((xy[v[i]][0] - xy[v[i + 1]][0]) * (xy[v[i]][0] - xy[v[i + 1]][0])
                + (xy[v[i]][1] - xy[v[i + 1]][1]) * (xy[v[i]][1] - xy[v[i + 1]][1]))
                as f64)
                .sqrt();
        }
        count += 1;
    }
    println!("{}", sum / count as f64);
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
