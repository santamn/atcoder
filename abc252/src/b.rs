use proconio::*;
use std::collections::HashSet;

#[fastout]
fn main() {
    input! {
        n: usize, k: usize,
        tastes: [usize; n],
        dislike: [usize; k],
    }

    let max = *tastes.iter().max().unwrap();
    let mut bests = HashSet::new();
    for (i, taste) in tastes.iter().enumerate() {
        if *taste == max {
            bests.insert(i + 1);
        }
    }

    let dislike: HashSet<_> = dislike.into_iter().collect();
    println!(
        "{}",
        if bests.intersection(&dislike).collect::<Vec<_>>().len() > 0 {
            "Yes"
        } else {
            "No"
        }
    )
}
