use proconio::*;
use std::cmp::max;

fn main() {
    input! {
        n: usize, m: usize,
        a: [isize; n],
    }

    // dp[i][j]: A_iまでのうち,既にj個をBの要素として選んだ時の和の最大値
    let mut dp = vec![vec![std::isize::MIN / 4; m + 1]; n + 1];
    dp[0][0] = 0;
    for i in 1..=n {
        for j in 0..=m {
            if j == 0 {
                dp[i][0] = 0;
            } else if j > i {
                dp[i][j] = std::isize::MIN / 4;
            } else {
                dp[i][j] = max(dp[i - 1][j], dp[i - 1][j - 1] + (j as isize) * a[i - 1]);
            }
        }
    }

    println!("{}", dp[n][m])
}
