use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        discs: [isize; n],
    }

    let set: HashSet<_> = discs.into_iter().collect();
    println!("{}", set.len())
}
