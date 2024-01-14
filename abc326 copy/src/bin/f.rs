#[proconio::fastout]
fn main() {
    proconio::input!(n: usize, x: i64, y: i64, a: [i64; n]);

    if n == 1 {
        println!("No");
        return;
    }

    let mut odd_set_1 = std::collections::HashMap::new();
    odd_set_1.insert(0, vec![]);
    for i in 0..n / 2 / 2 {
        let mut buff = std::collections::HashMap::new();
        for (j, v) in odd_set_1.iter() {
            buff.insert(j + a[2 * i + 1], v.clone());
            buff.get_mut(&(j + a[2 * i + 1])).unwrap().push(0u8);
            buff.insert(j - a[2 * i + 1], v.clone());
            buff.get_mut(&(j - a[2 * i + 1])).unwrap().push(1);
        }

        odd_set_1 = buff;
    }

    let mut odd_set_2 = std::collections::HashMap::new();
    odd_set_2.insert(0, vec![]);
    for i in n / 2 / 2..n / 2 {
        let mut buff = std::collections::HashMap::new();
        for (j, v) in odd_set_2.iter() {
            buff.insert(j + a[2 * i + 1], v.clone());
            buff.get_mut(&(j + a[2 * i + 1])).unwrap().push(0u8);
            buff.insert(j - a[2 * i + 1], v.clone());
            buff.get_mut(&(j - a[2 * i + 1])).unwrap().push(1);
        }

        odd_set_2 = buff;
    }

    let mut odd_v = vec![];
    for (&k, v) in odd_set_1.iter() {
        if let Some(x) = odd_set_2.get(&(x - k)) {
            odd_v = v.clone();
            odd_v.append(&mut x.clone());
        }
    }
    if odd_v.is_empty() {
        println!("No");
        return;
    }

    let mut even_set_1 = std::collections::HashMap::new();
    even_set_1.insert(0, vec![]);
    for i in 0..(n + 1) / 2 / 2 {
        let mut buff = std::collections::HashMap::new();
        for (j, v) in even_set_1.iter() {
            buff.insert(j + a[2 * i], v.clone());
            buff.get_mut(&(j + a[2 * i])).unwrap().push(0u8);
            buff.insert(j - a[2 * i], v.clone());
            buff.get_mut(&(j - a[2 * i])).unwrap().push(1);
        }

        even_set_1 = buff;
    }

    let mut even_set_2 = std::collections::HashMap::new();
    even_set_2.insert(0, vec![]);
    for i in (n + 1) / 2 / 2..(n + 1) / 2 {
        let mut buff = std::collections::HashMap::new();
        for (j, v) in even_set_2.iter() {
            buff.insert(j + a[2 * i], v.clone());
            buff.get_mut(&(j + a[2 * i])).unwrap().push(0u8);
            buff.insert(j - a[2 * i], v.clone());
            buff.get_mut(&(j - a[2 * i])).unwrap().push(1);
        }

        even_set_2 = buff;
    }

    let mut even_v = vec![];
    for (&k, v) in even_set_1.iter() {
        if let Some(x) = even_set_2.get(&(y - k)) {
            even_v = v.clone();
            even_v.append(&mut x.clone());
        }
    }
    if even_v.is_empty() {
        println!("No");
        return;
    }
    let mut ans = vec![];

    let mut lr = 0;
    for i in 0..(n + 1) / 2 {
        let buff = if (lr == 0 && even_v[i] == 0) || (lr == 1 && even_v[i] == 1) {
            'L'
        } else {
            'R'
        };
        ans.push(buff);

        if odd_v.len() == i {
            break;
        }

        let buff_2 = if (lr == 0 && buff == 'L' && odd_v[i] == 0)
            || (lr == 0 && buff == 'R' && odd_v[i] == 1)
            || (lr == 1 && buff == 'R' && odd_v[i] == 0)
            || (lr == 1 && buff == 'L' && odd_v[i] == 1)
        {
            'R'
        } else {
            'L'
        };

        ans.push(buff_2);

        lr = odd_v[i];
    }

    println!("Yes");
    println!("{}", ans.iter().collect::<String>());
}
