use std::{cmp::min, vec};

use proconio::*;

trait AbsDiff {
    fn abs_diff(self, other: usize) -> usize
    where
        Self: Sized;
}

impl AbsDiff for usize {
    fn abs_diff(self, other: usize) -> usize
    where
        Self: Sized,
    {
        if self > other {
            self - other
        } else {
            other - self
        }
    }
}

fn main() {
    input! {
        n: usize, k: usize,
        h: [usize; n],
    }

    // dp[i]: 足場i-1に登るための最小コスト
    let mut dp = vec![std::usize::MAX / 4; n];
    dp[0] = 0;
    for i in 1..n {
        dp[i] = (1..=min(i, k))
            .map(|j| dp[i - j] + h[i - j].abs_diff(h[i]))
            .min()
            .unwrap();
    }

    println!("{}", dp[n - 1]);
}
