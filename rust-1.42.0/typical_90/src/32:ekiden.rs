use itertools::Itertools;
use proconio::input;
use proconio::marker::Usize1;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        time: [[isize; n]; n],
        m: usize,
        rumors: [(Usize1, Usize1); m],
    }

    let discord_pairs: HashSet<(usize, usize)> = rumors
        .into_iter()
        .map(|(x, y)| vec![(x, y), (y, x)])
        .flatten()
        .collect();

    let fastest: isize = (0..n)
        .permutations(n)
        .filter(|perm| {
            perm.windows(2)
                .all(|pair| !discord_pairs.contains(&(pair[0], pair[1])))
        })
        .map(|perm| {
            perm.iter()
                .enumerate()
                .map(|(section, &runner)| time[runner][section])
                .sum()
        })
        .min()
        .unwrap_or(-1);

    println!("{}", fastest);
}
