use itertools::Itertools;
use proconio::*;

fn main() {
    input! {
        n: usize,
        m: usize,
        s: [String; n],
        t: [String; m],
    }

    for shuffled_s in s.iter().permutations(n) {}
}
