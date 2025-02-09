use itertools::Itertools;
use proconio::{marker::Usize1, *};
use std::collections::HashMap;

#[fastout]
fn main() {
    input! {
        n: usize,
        p: [Usize1; n],
        q: [Usize1; n],
    }

    let rev_q = q
        .iter()
        .enumerate()
        .map(|(i, &x)| (x, i))
        .collect::<HashMap<_, _>>();

    println!(
        "{}",
        (0..n).map(|i| (q[p[rev_q[&i]]] + 1).to_string()).join(" ")
    );
}
