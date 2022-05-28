use proconio::marker::Chars;
use proconio::*;

// スロットをある数字iで揃えるのにかかる時間t = (数字iが各スロットで一番被ってる箇所の被ってる個数 - 1)*10 + 被ってる場所の番目j

#[fastout]
fn main() {
    input! {
        n: usize,
        reels: [Chars; n],
    }

    let numbers: Vec<Vec<_>> = reels
        .iter()
        .map(|reel| reel.iter().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    let mut duplicated = vec![vec![0; 10]; 10]; // duplicated[i][j]: 数字iが各スロットのj番目において被っている数
    for m in 0..=9 {
        for i in 0..n {
            for j in 0..10 {
                if m == numbers[i][j] {
                    duplicated[m as usize][j] += 1;
                }
            }
        }
    }

    let time = duplicated
        .iter()
        .map(|list| {
            list.iter().enumerate().fold(0, |acc, (i, &val)| {
                let t = (val as isize - 1) * 10 + i as isize;
                if t > acc {
                    t
                } else {
                    acc
                }
            })
        })
        .min()
        .unwrap();

    println!("{}", time);
}
