use proconio::*;

fn main() {
    input! {
        n: isize,
    }

    println!("{}", n.rem_euclid(998244353));
}
