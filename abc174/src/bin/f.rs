use my_crate::structure::BIT::BIT;
use my_crate::structure::BIT::Sum;
#[allow(unused_imports)]
use proconio::{fastout, input};
 
#[fastout]
fn main() {
    input!(n: usize, q: usize, c: [usize; n], lr: [[usize; 2]; q]);

    let mut c_vv = vec![vec![0usize; 0]; n];
    let mut c_v = vec![0i64; n + 1];
    for (i, v) in c.iter().enumerate() {
        let l = c_v[*v];
        if l != -1 {
            c_vv[l as usize].push(i);
        }
        c_v[*v] = i as i64;
    }

    let mut lr_vec = vec![vec![vec![0usize; 2];0]; q];
    for (i,v) in lr.iter().enumerate() {
        lr_vec[v[0]].push( vec![v[1],i]);
    }

    let mut ans = vec![0usize;q];
    let mut bi_tree = BIT::<usize>::new(n); 
    for i in (0..n).rev(){
        for cv in &c_vv[i]{
            bi_tree.add(*cv, 1);
        }

        for v in &lr_vec[i]{
             ans[v[1]] = (v[0]-i+1)-bi_tree.sum(v[0]);
        }
 
        for v in &ans{
            println!("{}",*v);
        }



    }


    println!("{:?}", lr_vec);
}
