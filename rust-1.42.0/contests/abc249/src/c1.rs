use itertools::Itertools;
use proconio::*;
use std::cmp;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize, k:usize,
        str: [String; n],
    }

    let mut frequency = HashMap::<char, usize>::new();
    for s in str.iter() {
        for c in s.chars() {
            *frequency.entry(c).or_insert(0) += 1;
        }
    }
    // 頻度kの文字種がいくつあるか
    let count = frequency
        .iter()
        .fold(0, |r, (_, freq)| if *freq == k { r + 1 } else { r });

    println!("{}", count);
}
