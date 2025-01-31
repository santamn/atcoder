use std::vec;

use proconio::*;

fn main() {
    input! {
        n: usize,
        leisure: [[isize; 3]; n],
    }

    // dp[i][j]: i日目に活動jをした時のそれまでの幸福度の累積の最大値
    let mut dp = vec![vec![0; 3]; n + 1];

    for i in 1..=n {
        for j in 0..3 {
            dp[i][j] += (0..3)
                .filter(|&x| x != j)
                .map(|x| dp[i - 1][x] + leisure[i - 1][x])
                .max()
                .unwrap();
        }
    }

    println!("{}", dp[n].iter().max().unwrap())
}
