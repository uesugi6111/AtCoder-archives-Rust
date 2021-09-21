fn main() {
    for i in 1..30 {
        println!("{} : {}", i, solution(i as i32));
    }
}

pub fn solution(n: i32) -> i32 {
    // TODO: implement me!
    let n = n as usize;

    let mut dp = vec![0; n + 2];

    dp[0] = 1;

    for i in 0..n {
        dp[i + 1] += dp[i];
        dp[i + 2] += dp[i];
    }
    dp[n]
}

// よく見たらフィボナッチ数
