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
        n: usize,
        h: [usize; n],
    }

    // dp[i]: 足場i-1に登るための最小コスト
    let mut dp = vec![std::usize::MAX / 4; n];
    dp[0] = 0;
    dp[1] = h[0].abs_diff(h[1]);

    for i in 2..n {
        dp[i] = min(
            dp[i - 2] + h[i - 2].abs_diff(h[i]),
            dp[i - 1] + h[i - 1].abs_diff(h[i]),
        )
    }

    println!("{}", dp[n - 1]);
}
