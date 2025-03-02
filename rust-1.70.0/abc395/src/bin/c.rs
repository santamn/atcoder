use proconio::{fastout, input};
use std::collections::hash_map::Entry;
use std::collections::HashMap;

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut pos = HashMap::new();

    println!(
        "{}",
        a.iter()
            .enumerate()
            .filter_map(|(i, &a)| match pos.entry(a) {
                Entry::Occupied(mut e) => Some((i - e.insert(i) + 1) as isize),
                Entry::Vacant(e) => {
                    e.insert(i);
                    None
                }
            })
            .min()
            .unwrap_or(-1)
    )
}
