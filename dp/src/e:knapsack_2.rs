use std::cmp::min;

use proconio::*;

const VALUE_SUM: usize = 100_000;

fn main() {
    input! {
        n: usize, w: usize
    }

    let mut weights = vec![0; n + 1];
    let mut values = vec![0; n + 1];
    for i in 1..=n {
        input! {weight: usize, value: usize}
        weights[i] = weight;
        values[i] = value;
    }

    // dp[i][j]: i番目までの品物を使って価値をjにする時の重さの最小値
    let mut dp = vec![vec![std::usize::MAX / 4; VALUE_SUM + 1]; n + 1];
    dp[0][0] = 0;
    for i in 1..=n {
        for j in 0..=VALUE_SUM {
            dp[i][j] = if j >= values[i] {
                min(dp[i - 1][j], dp[i - 1][j - values[i]] + weights[i])
            } else {
                dp[i - 1][j]
            }
        }
    }

    println!("{}", dp[n].iter().rposition(|&weight| weight <= w).unwrap())
}
