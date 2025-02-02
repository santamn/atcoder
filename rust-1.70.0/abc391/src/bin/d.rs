use itertools::Itertools;
use proconio::{marker::Usize1, *};

#[fastout]
fn main() {
    input_interactive! {
        n: usize, w: usize,
        blocks: [(usize, usize); n], // (X, Y)
        q: usize,
    }

    // 各列にあるブロックをY座標でソート
    // blocks_in_column[x]: x列目にあるブロックの、番号とy座標のリスト
    let mut blocks_in_column: Vec<Vec<(usize, usize)>> = vec![Vec::new(); w + 1];
    blocks
        .clone()
        .into_iter()
        .enumerate()
        .sorted_by(|a, b| a.1.cmp(&b.1)) // 各ブロックを辞書式順序(X座標でソートした上で、Y座標でソート)で並べる
        .for_each(|(block, (x, y))| {
            blocks_in_column[x].push((block, y));
        });

    // ranks[block]: block番目のブロックがその列で下から何番目にあるか
    // max_heights[rank]: あるrankのブロックの中で、最も高い位置にあるブロックのY座標
    // counts[rank]: あるrankのブロックの数
    let (ranks, max_heights, counts): (Vec<usize>, Vec<usize>, Vec<usize>) = blocks_in_column
        .into_iter()
        .flat_map(|blocks_and_heights| {
            // 各列ごとに、下から順位をつける
            blocks_and_heights
                .into_iter()
                .enumerate()
                .map(|(rank, (block, y))| (block, rank, y))
        })
        .fold(
            (vec![0; n], vec![0; n], vec![0; n]),
            |(mut ranks, mut max_heights, mut counts), (block, rank, y)| {
                ranks[block] = rank;
                max_heights[rank] = max_heights[rank].max(y);
                counts[rank] += 1;
                (ranks, max_heights, counts)
            },
        );

    for _ in 0..q {
        input_interactive! {
            t: usize, block: Usize1,
        }

        // 時刻tにblock番目のブロックが残っているか
        println!(
            "{}",
            if max_heights[ranks[block]] > t || counts[ranks[block]] != w {
                "Yes"
            } else {
                "No"
            }
        );
    }
}
