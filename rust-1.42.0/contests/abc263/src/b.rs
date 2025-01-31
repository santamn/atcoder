use proconio::marker::Usize1;
use proconio::*;

#[fastout]
fn main() {
    input! {
        n: usize,
        p: [Usize1; n-1],
    }

    let mut count = 1;
    let mut i = n - 2;
    while p[i] != 0 {
        count += 1;
        i = p[i] - 1;
    }

    println!("{}", count);
}
