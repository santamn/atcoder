use itertools::Itertools;
use proconio::*;
use std::collections::HashSet;

#[fastout]
fn main() {
    input! {
        n: usize, m: usize,
        a: [usize; m],
    }

    // 1 <= a <= n
    // aに含まれないn以下の数値を昇順に調べる
    let b = a.into_iter().collect::<HashSet<_>>();
    let ans = (1..=n).filter(|x| !b.contains(x)).collect::<Vec<usize>>();

    // 答えを出力
    println!("{}", ans.len());
    println!("{}", ans.into_iter().join(" "));
}
