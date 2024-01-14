#[proconio::fastout]
fn main() {
    proconio::input!(a: [[i64; 9]; 9]);

    for i in 0..9 {
        let sum = a[i]
            .iter()
            .copied()
            .collect::<std::collections::HashSet<_>>();
        if sum.len() != 9 {
            println!("No");
            return;
        }
    }

    for i in 0..9 {
        let sum = (0..9)
            .map(|x| a[x][i])
            .collect::<std::collections::HashSet<_>>();
        if sum.len() != 9 {
            println!("No");
            return;
        }
    }

    for i in 0..3 {
        for j in 0..3 {
            let mut sum = std::collections::HashSet::new();
            for k in 0..3 {
                for l in 0..3 {
                    sum.insert(a[i * 3 + k][j * 3 + l]);
                }
            }
            if sum.len() != 9 {
                println!("No");
                return;
            }
        }
    }
    println!("Yes");
}


kdtree::KdTree