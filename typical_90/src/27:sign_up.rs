use proconio::*;
use std::collections::HashSet;

#[fastout]
fn main() {
    input! {
        n: usize,
        s: [String; n],
    }

    let mut hashset = HashSet::new();
    let mut answer = Vec::new();
    for (i, name) in s.iter().enumerate() {
        if !hashset.contains(name) {
            answer.push(i + 1);
            hashset.insert(name);
        }
    }

    for v in answer {
        println!("{}", v);
    }
}
