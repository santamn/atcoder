use proconio::{fastout, input, marker::Usize1};
use std::collections::HashSet;
#[fastout]
fn main() {
    input! {
        _: usize, m: usize,
        edges: [(Usize1, Usize1); m],
    }

    let mut set = HashSet::new();
    let mut count = 0;
    for (u, v) in edges {
        if set.contains(&(u, v)) || u == v {
            count += 1;
        } else {
            set.insert((u, v));
            set.insert((v, u));
        }
    }
    println!("{}", count);
}
