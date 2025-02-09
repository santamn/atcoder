use itertools::Itertools;
use proconio::*;

#[fastout]
fn main() {
    input! {
        a: usize, b: usize, c: usize,
    }

    let &[min, middle, max] = &[a, b, c].into_iter().sorted().collect::<Vec<_>>()[..] else {
        unreachable!();
    };
    if max == middle * min {
        println!("Yes");
    } else {
        println!("No");
    }
}
