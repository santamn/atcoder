use num::integer::gcd;
use proconio::*;

#[fastout]
fn main() {
    input! {
        a: usize, b: usize, c: usize,
    }

    let gcd = gcd(a, gcd(b, c));
    let cut = a / gcd + b / gcd + c / gcd - 3;
    println!("{}", cut);
}
