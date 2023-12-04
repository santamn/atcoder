use proconio::*;

fn main() {
    input! {
        n: usize, p: usize, q: usize, r: usize,
        a: [usize; n],
    }

    let mut cumsum: Vec<usize> = vec![0];
    cumsum.append(
        &mut a
            .iter()
            .scan(0, |cum, x| {
                *cum += x;
                Some(*cum)
            })
            .collect(),
    );

    for x in 0..n - 1 {
        if let Ok(y) = cumsum.binary_search(&(cumsum[x] + p)) {
            if let Ok(z) = cumsum.binary_search(&(cumsum[y] + q)) {
                if let Ok(_) = cumsum.binary_search(&(cumsum[z] + r)) {
                    println!("Yes");
                    return;
                }
            }
        }
    }
    println!("No");
}
