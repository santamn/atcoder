use proconio::*;
use std::cmp::max;

fn main() {
    input! {
        n: usize, weight_limit: usize,
    }

    let mut W = vec![usize::default(); n + 1];
    let mut V = vec![usize::default(); n + 1];
    for i in 1..=n {
        input! {w: usize, v: usize}
        W[i] = w;
        V[i] = v;
    }

    // dp[i][j]=i番目までの品物を、重さがj以下になるように選んだときの価値の最大値
    let mut dp = vec![vec![0usize; weight_limit + 1]; n + 1];
    for i in 1..=n {
        for j in 1..=weight_limit {
            dp[i][j] = if j >= W[i] {
                max(dp[i - 1][j], dp[i - 1][j - W[i]] + V[i])
            } else {
                dp[i - 1][j]
            }
        }
    }

    println!("{}", dp[n][weight_limit])
}
