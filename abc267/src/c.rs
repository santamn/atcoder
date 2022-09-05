use proconio::*;

fn main() {
    input! {
        n: usize, m: usize,
        a: [isize; n],
    }

    let mut cumsum: Vec<isize> = vec![0];
    cumsum.append(
        &mut a
            .iter()
            .scan(0, |cum, x| {
                *cum += x;
                Some(*cum)
            })
            .collect(),
    );

    let mut c = vec![0; n - m + 1];
    c[0] = a[..m]
        .iter()
        .enumerate()
        .map(|(i, v)| (i as isize + 1) * v)
        .sum();

    for i in 0..(n - m) {
        c[i + 1] = c[i] + (m as isize) * a[i + m] - a[i] - (cumsum[i + m] - cumsum[i + 1]);
    }

    println!("{}", c.iter().max().unwrap());
}
