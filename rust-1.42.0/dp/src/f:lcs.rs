use proconio::{marker::Chars, *};
use std::cmp::max;

fn main() {
    input! {
        s: Chars,
        t: Chars,
    }

    // dp[i][j]: tをi文字目まで、sをj文字目まで使ってできる最長の部分文字列の長さ
    let mut dp: Vec<Vec<isize>> = vec![vec![std::isize::MIN; s.len() + 1]; t.len() + 1];
    dp[0] = vec![0; s.len() + 1];
    for i in 0..=t.len() {
        dp[i][0] = 0;
    }

    for i in 1..=t.len() {
        for j in 1..=s.len() {
            dp[i][j] = if t[i - 1] == s[j - 1] {
                dp[i - 1][j - 1] + 1
            } else {
                max(dp[i - 1][j], dp[i][j - 1])
            }
        }
    }

    let mut length = dp[t.len()][s.len()] as usize;
    let mut i = t.len();
    let mut j = s.len();
    let mut answer = vec![char::default(); length];
    while length > 0 {
        if t[i - 1] == s[j - 1] {
            answer[length - 1] = t[i - 1];
            length -= 1;
            i -= 1;
            j -= 1;
        } else {
            if dp[i][j] == dp[i - 1][j] {
                i -= 1;
            } else {
                j -= 1;
            }
        }
    }

    println!("{}", answer.into_iter().collect::<String>())
}
