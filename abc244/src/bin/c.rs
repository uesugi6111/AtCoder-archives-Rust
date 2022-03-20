fn main() {
    let n: usize = readln();

    let mut set = std::collections::HashSet::new();

    let mut count = 1;

    while set.len() != 2 * (n + 1) {
        while set.contains(&count) {
            count += 1;
        }
        println!("{}", count);
        set.insert(count);

        let x: i64 = readln();
        set.insert(x);
    }
}

fn get_line() -> String {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().to_string()
}
fn readln<T>() -> T
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    get_line().parse().unwrap()
}
