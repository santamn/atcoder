use proconio::*;
use std::cmp::max;

fn main() {
    input! {
        r: isize,
        c: isize,
    }

    let x = (r - 8).abs();
    let y = (c - 8).abs();

    if max(x, y) % 2 == 0 {
        println!("white");
    } else {
        println!("black");
    }
}
