use proconio::*;

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        mat: [[usize; w]; h],
    }

    let horizontal: Vec<usize> = mat.iter().map(|vec| vec.iter().sum()).collect();
    let mut vertical: Vec<usize> = vec![0; w];
    for j in 0..w {
        let mut sum = 0;
        for i in 0..h {
            sum += mat[i][j];
        }
        vertical[j] = sum;
    }

    for i in 0..h {
        for j in 0..w {
            print!("{} ", horizontal[i] + vertical[j] - mat[i][j]);
        }
        println!("")
    }
}
