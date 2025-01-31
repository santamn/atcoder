use num::integer::lcm;
use proconio::*;

#[fastout]
fn main() {
    input! {
        n: u128, a:u128, b:u128
    }

    let m = n / a;
    let l = n / b;
    let k = n / lcm(a, b);

    let ans =
        n * (n + 1) / 2 - a * m * (m + 1) / 2 - b * l * (l + 1) / 2 + lcm(a, b) * k * (k + 1) / 2;
    println!("{}", ans);
}
