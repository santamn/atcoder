use proconio::*;
use std::{collections::HashSet, iter::FromIterator};

fn main() {
    input! {
        n: usize, p: isize, q: isize, r: isize,
        a: [isize; n],
    }

    let mut cumsum: Vec<isize> = vec![0];
    cumsum.append(
        &mut a
            .iter()
            .scan(0, |cum, x| {
                *cum += x;
                Some(*cum)
            })
            .collect(),
    );

    let set: HashSet<isize> = HashSet::from_iter(cumsum.clone().into_iter());
    for i in 0..n {
        if set.contains(&(cumsum[i + 1] - r))
            && set.contains(&(cumsum[i + 1] - r - q))
            && set.contains(&(cumsum[i + 1] - r - q - p))
        {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
