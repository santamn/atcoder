use itertools::Itertools;
use proconio::{fastout, input, marker::Chars};
use std::iter;

#[fastout]
fn main() {
    input! {
        _: usize, k: usize,
        s: Chars,
    }

    // o　と隣り合う ? は . にする
    let s = iter::once('.')
        .chain(s.into_iter())
        .chain(iter::once('.'))
        .tuple_windows()
        .map(|(a, b, c)| match (a, b, c) {
            ('o', '?', _) | (_, '?', 'o') => '.',
            (_, b, _) => b,
        })
        .collect::<Vec<_>>();

    let least_o = s.iter().filter(|&&c| c == 'o').count();
    if least_o == k {
        // 隣り合う o の情報のみで ? -> . とした段階で、すでに o の数が k と等しい場合
        // 残りの ? -> . と置き換えた文字列を出力
        println!(
            "{}",
            s.iter()
                .map(|&c| if c == '?' { '.' } else { c })
                .collect::<String>()
        );
    } else {
        // o の数がまだ k より少ない場合

        if s.iter()
            .group_by(|&&c| c == '?')
            .into_iter()
            .flat_map(|(is_q, chunk)| {
                let c = chunk.count();
                is_q.then_some((c >> 1) + (c & 1)) // o.o.o.o のようにoを配置すると考えて o の数をカウント
            })
            .sum::<usize>()
            + least_o
            == k
        {
            // ? の数を最大限 o に置き換える場合
            // ????? の塊の長さが奇数個のところは、 o.o.o となる
            // 長さが偶数個の場合は ???? のままでよい
            println!(
                "{}",
                s.iter()
                    .group_by(|&&c| c == '?')
                    .into_iter()
                    .map(|(is_q, chunk)| {
                        let run = chunk.collect::<String>();
                        if is_q && run.len() & 1 == 1 {
                            ['o', '.']
                                .into_iter()
                                .cycle()
                                .take(run.len())
                                .collect::<String>()
                        } else {
                            run
                        }
                    })
                    .collect::<String>()
            );
        } else {
            // 最大限 ? を o に置き換える必要がない場合
            println!("{}", s.iter().collect::<String>());
        }
    }
}
