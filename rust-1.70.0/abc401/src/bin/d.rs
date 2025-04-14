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

    if s.iter().filter(|&&c| c == 'o').count() == k {
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
            .map(|(question, chunk)| {
                if question {
                    let c = chunk.count();
                    (c >> 1) + (c & 1) // o.o.o.o のようにoを配置する
                } else {
                    chunk.into_iter().filter(|&&c| c == 'o').count()
                }
            })
            .sum::<usize>()
            == k
        {
            // ? の数を最大限 o に置き換えた場合、o の数が k と等しくなる場合
            // ????? の塊の長さが奇数個のところは、 o.o.o となる
            // 長さが偶数個の場合は ???? のままでよい
            println!(
                "{}",
                s.iter()
                    .group_by(|&&c| c == '?')
                    .into_iter()
                    .map(|(question, chunk)| {
                        let run = chunk.collect::<String>();
                        if question && run.len() & 1 == 1 {
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
