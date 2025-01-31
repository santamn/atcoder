use proconio::*;
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

    for i in 0..k * n {
        if !func(&frequency, i, k) {
            println!("{}", i - 1);
            return;
        }
    }
}

// e個の文字がk個以上の文字列に含まれるかを判定する
fn func(freq: &HashMap<char, usize>, e: usize, k: usize) -> bool {
    let mut num = 0;
    for (_, &count) in freq {
        if count >= k {
            num += 1
        }
    }
    num >= e
}
