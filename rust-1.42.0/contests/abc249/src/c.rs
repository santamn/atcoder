use proconio::*;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize, k: usize,
        str: [String; n],
    }

    // bit全探索
    let mut max = 0;
    for bit in 0..(1 << n) {
        let mut frequency = HashMap::<char, usize>::new();

        for s in (0..n).filter(|x| bit & (1 << x) != 0).map(|x| &str[x]) {
            // 各文字の出現頻度を計算
            for c in s.chars() {
                *frequency.entry(c).or_insert(0) += 1
            }
        }
        // 頻度kの文字種がいくつあるか
        let count = frequency
            .iter()
            .fold(0, |c, (_, &freq)| if freq == k { c + 1 } else { c });
        max = std::cmp::max(max, count);
    }

    println!("{}", max);
}
