use proconio::*;

fn main() {
    input! {
        a: usize, b: usize, c: usize, d: usize, e: usize,
    }

    let mut v = vec![a, b, c, d, e];
    v.sort();
    v.dedup();
    println!("{}", v.len())
}
