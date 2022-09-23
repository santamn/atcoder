use itertools::Itertools;
use proconio::*;

fn main() {
    input! {
        n: usize,
    }

    // 64-n.leading_zeros(): 型のbit数 - leading_zeros() で2進数での桁数がわかる
    let w = (0..(64 - n.leading_zeros()))
        .map(|i| n & 1 << i)
        .enumerate()
        .filter(|&(_, m)| m != 0)
        .map(|(i, _)| i)
        .collect_vec();

    let mut answer = (0..=w.len())
        .map(|i| {
            w.iter()
                .combinations(i)
                .map(|v| v.iter().fold(0, |acc, &&x| acc + (1usize << x)))
        })
        .flatten()
        .collect_vec();
    answer.sort();

    for v in answer {
        println!("{}", v);
    }
}
