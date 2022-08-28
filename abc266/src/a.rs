use proconio::marker::Chars;
use proconio::*;

fn main() {
    input! {
        s: Chars,
    }

    println!("{}", s[(s.len() + 1) / 2 - 1]);
}
