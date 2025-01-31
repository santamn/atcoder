use proconio::marker::Usize1;
use proconio::*;

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [Usize1; n],
    }

    let mut count1 = 0i64;
    let mut count2 = 0i64;
    for i in 0..n {
        if a[i] == i {
            count1 += 1;
        }
    }

    for i in 0..n {
        if a[i] > i && a[a[i]] == i {
            count2 += 1;
        }
    }

    println!("{}", (count1 * (count1 - 1)) / 2 + count2);
}
