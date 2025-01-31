use itertools::Itertools;
use proconio::*;

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
    }

    for comb in (1..=m).combinations(n) {
        for v in comb {
            print!("{} ", v);
        }
        println!()
    }
}
