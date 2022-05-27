use proconio::*;
use std::cmp;

const INF: usize = 100_000;

#[fastout]
fn main() {
    input! {
        n: usize,
        a: usize, b: usize, c: usize,
    }

    let v = vec![a, b, c];

    // dp[i][j]: i番目までのコインを使ってj円払うときの最小枚数
    let mut dp = vec![vec![INF; n + 1]; 4];
    for i in 0..=3 {
        // i枚のコインで0円を払う時の使用枚数は0
        dp[i][0] = 0;
    }

    for (i, &coin) in v.iter().enumerate() {
        for j in 1..=n {
            if j >= coin {
                dp[i + 1][j] = cmp::min(dp[i + 1][j - coin] + 1, dp[i][j]);
            } else {
                dp[i + 1][j] = dp[i][j];
            }
        }
    }

    println!("{}", dp[3][n]);
}
