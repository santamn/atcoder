use proconio::marker::Chars;
use proconio::*;

const MOD: usize = 1_000_000_007;

// もらうDP
#[fastout]
fn main() {
    input! {
        n: usize,
        s: Chars
    }
    let atcoder: Vec<char> = "atcoder".chars().collect();

    // dp[i][j]: sの[0,i)文字目を使って"atcoder"のj文字目までを作る場合の数
    let mut dp: Vec<Vec<usize>> = vec![vec![0; 8]; n + 1];
    for i in 0..=n {
        dp[i][0] = 1;
    }

    for (i, c) in s.iter().enumerate() {
        for j in 0..7 {
            if *c == atcoder[j] {
                dp[i + 1][j + 1] = (dp[i][j] + dp[i][j + 1]) % MOD;
            } else {
                dp[i + 1][j + 1] = dp[i][j + 1] % MOD;
            }
        }
    }

    println!("{}", dp[n][atcoder.len()])
}
