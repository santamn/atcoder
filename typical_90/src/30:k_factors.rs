use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
    }

    let mut factors = vec![0usize; n + 1];
    for i in 2..=n {
        if factors[i] > 0 {
            continue;
        }

        for j in (i..=n).step_by(i) {
            factors[j] += 1;
        }
    }

    println!("{}", factors.into_iter().filter(|&num| num >= k).count())
}
