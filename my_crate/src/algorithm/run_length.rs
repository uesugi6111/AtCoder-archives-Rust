pub fn compress(s: &[char]) -> Vec<(char, usize)> {
    let mut buff = s[0];
    let mut count = 1;
    let mut v = vec![];
    for c in s {
        if *c != buff {
            v.push((buff, count));
            count = 1;
            buff = *c;
        } else {
            count += 1;
        }
    }
    v
}
