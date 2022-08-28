use proconio::*;
use std::cmp::max;

const T_MAX: usize = 100_000;

fn main() {
    input! { n: usize }

    let mut position = vec![0; T_MAX + 1];
    let mut length = vec![0; T_MAX + 1];

    for _ in 0..n {
        input! { t: usize, x: usize, a: isize}
        position[t] = x;
        length[t] = a;
    }

    // dp[x][t]: 時間tにxにいるときに、それまでに捕まえなsnukeの大きさの合計の最大値
    let mut dp = vec![vec![std::isize::MIN / 4; T_MAX + 1]; 5];
    dp[0][0] = 0;
    for t in 1..=T_MAX {
        for x in 0..=4 {
            dp[x][t] = dp[x][t - 1];
            if x != 0 {
                dp[x][t] = max(dp[x - 1][t - 1], dp[x][t]);
            }
            if x != 4 {
                dp[x][t] = max(dp[x + 1][t - 1], dp[x][t]);
            }
        }
        dp[position[t]][t] += length[t];
    }

    println!(
        "{}",
        vec![
            dp[0][T_MAX],
            dp[1][T_MAX],
            dp[2][T_MAX],
            dp[3][T_MAX],
            dp[4][T_MAX],
        ]
        .into_iter()
        .max()
        .unwrap()
    )
}
