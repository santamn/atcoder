use proconio::*;
use std::cmp::max;

#[fastout]
fn main() {
    input! {
        n: usize,
        l: isize,
        r: isize,
        a: [isize; n],
    }

    let left: Vec<isize> = a
        .iter()
        .scan(0, |diff, &x| {
            *diff += x - l;
            Some(*diff)
        })
        .collect();

    let right: Vec<isize> = a
        .iter()
        .rev()
        .scan(0, |diff, &x| {
            *diff += x - r;
            Some(*diff)
        })
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .collect();

    // right_max[i] = max right[i..=N]
    let right_max: Vec<isize> = right
        .iter()
        .rev()
        .scan(std::isize::MIN, |a, &b| {
            *a = max(*a, b);
            Some(*a)
        })
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .collect();

    let mut gain = right_max[0];
    for i in 0..n - 1 {
        gain = max(gain, max(left[i], 0) + max(right_max[i + 1], 0))
    }
    gain = max(gain, left[n - 1]);

    println!("{}", a.iter().sum::<isize>() - gain);
}
