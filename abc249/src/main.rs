use proconio::*;

const MAX: usize = 200_000;

#[fastout]
fn main() {
    input! { n: usize }
    let mut arr = vec![0; MAX + 1];

    for _ in 0..n {
        input! { a: usize }
        arr[a] += 1;
    }

    let mut sum: u64 = 0;
    for q in 1..=MAX {
        let mut r = 1;
        while q * r <= MAX {
            sum += arr[q] * arr[r] * arr[q * r];
            r += 1;
        }
    }

    println!("{}", sum);
}
