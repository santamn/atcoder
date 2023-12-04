use proconio::marker::Chars;
use proconio::*;

// 全ての数字0-9でスロットを揃えるシミュレーションを行う

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

    let mut answer = vec![0; 10]; // n=0-9それぞれについて、t秒でスロット全てをnで揃えて止めることができる
    for m in 0..=9 {
        let mut t = 0; // 時間t
        let mut stopped = vec![false; n]; // 第i番目のスロットが止まっているかどうか
        loop {
            for i in 0..n {
                // 現在のスロットの値のうち、揃えようとしている数値mに一致するスロットを探索
                if m == numbers[i][t % 10] && !stopped[i] {
                    stopped[i] = true;
                    break;
                }
            }

            // 全てのスロットが停止したら数mについてのシミュレーションを終了
            if stopped.iter().all(|&is_stopped| is_stopped) {
                answer[m as usize] = t;
                break;
            }
            t += 1;
        }
    }

    println!("{}", answer.iter().min().unwrap())
}
