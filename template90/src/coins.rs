use proconio::*;

const MAX: usize = 10000;

#[fastout]
fn main() {
    input! {
        n: usize,
        a: usize, b: usize, c: usize
    }

    let mut min = std::usize::MAX;
    for i in 0..MAX {
        if a * i > n {
            continue;
        }

        for j in 0..(MAX - i) {
            if a * i + b * j > n {
                continue;
            }

            if (n - (a * i + b * j)) % c == 0 {
                let k = (n - (a * i + b * j)) / c;
                min = std::cmp::min(min, i + j + k);
            }
        }
    }

    println!("{}", min);
}
