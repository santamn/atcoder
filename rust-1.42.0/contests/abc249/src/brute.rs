use proconio::*;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize, _: usize,
        str: [String; n],
    }

    for bit in 0..(1 << n) {
        print!("{{");
        for x in (0..n).filter(|x| bit & (1 << x) != 0).map(|x| &str[x]) {
            print!("{},", x);
        }
        println!("}}")
    }
}
