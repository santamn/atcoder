use proconio::marker::Chars;
use proconio::*;

#[fastout]
fn main() {
    input! {
        h: usize, w: usize,
        board: [Chars; h],
    }

    let mut vec = Vec::new();
    for i in 0..h {
        for j in 0..w {
            if board[i][j] == 'o' {
                vec.push((i as isize, j as isize));
            }
        }
    }

    println!(
        "{}",
        (vec[0].0 - vec[1].0).abs() + (vec[0].1 - vec[1].1).abs()
    )
}
