use itertools::Itertools;
use proconio::*;

fn main() {
    input! {
        h1: usize, w1: usize,
        a: [[usize; w1]; h1],
        h2: usize, w2: usize,
        b: [[usize; w2]; h2],
    }

    for rows in (0..h1).combinations(h2) {
        for cols in (0..w1).combinations(w2) {
            let mut is_equal = true;
            for i in 0..h2 {
                for j in 0..w2 {
                    if b[i][j] != a[rows[i]][cols[j]] {
                        is_equal = false;
                        break;
                    }
                }
            }

            if is_equal {
                println!("Yes");
                return;
            }
        }
    }

    println!("No")
}
