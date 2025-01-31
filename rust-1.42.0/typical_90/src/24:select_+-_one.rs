use proconio::*;

#[fastout]
fn main() {
    input! {
        n: usize, k: isize,
        a: [isize; n],
        b: [isize; n],
    }

    let sum: isize = a.iter().zip(b.iter()).map(|(a, b)| (a - b).abs()).sum();
    println!(
        "{}",
        if k >= sum && (k - sum) % 2 == 0 {
            "Yes"
        } else {
            "No"
        }
    )
}
