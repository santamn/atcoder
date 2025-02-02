use proconio::marker::Chars;
use proconio::*;

#[fastout]
fn main() {
    input! {
        n: usize,
        a: Chars,
    }

    let bits = a
        .into_iter()
        .map(|c| match c {
            '0' => 0,
            '1' => 1,
            _ => unreachable!(),
        })
        .collect::<Vec<_>>();

    let (_, c) = flips(n, 0, &bits);
    println!("{}", c);
}

fn flips(k: usize, i: usize, bits: &[usize]) -> (usize, usize) {
    if k == 0 {
        (bits[i], 1)
    } else {
        let (v1, c1) = flips(k - 1, 3 * i, bits);
        let (v2, c2) = flips(k - 1, 3 * i + 1, bits);
        let (v3, c3) = flips(k - 1, 3 * i + 2, bits);

        if v1 == v2 && v2 == v3 {
            (v1, c1 + c2 + c3 - [c1, c2, c3].iter().max().unwrap()) // c1 + c2 + c3 - max(c1, c2, c3) = min(c1 + c2, c1 + c3, c2 + c3)
        } else if v1 == v2 {
            (v1, c1.min(c2))
        } else if v2 == v3 {
            (v2, c2.min(c3))
        } else {
            // v1 == v3
            (v1, c1.min(c3))
        }
    }
}
