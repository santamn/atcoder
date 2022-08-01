use proconio::marker::Usize1;
use proconio::*;

#[fastout]
fn main() {
    input! {
        _: usize, n: usize,
        bricks: [(Usize1, Usize1); n],
    }

    let bricks = compress(&bricks);
    let mut heights = vec![0; 2 * n + 1];
    let mut answer = vec![0; n];
    for (i, &(l, r)) in bricks.iter().enumerate() {
        let (l, r) = if l < r { (l, r + 1) } else { (r, l + 1) };
        let &&max = &heights[l..r].iter().max().unwrap();
        answer[i] = max + 1;
        for j in l..r {
            heights[j] = max + 1;
        }
    }

    for h in answer {
        println!("{}", h);
    }
}

// 区間圧縮する関数
fn compress(bricks: &Vec<(usize, usize)>) -> Vec<(usize, usize)> {
    let mut arr: Vec<usize> = bricks.iter().map(|&(l, r)| vec![l, r]).flatten().collect();
    arr.sort();
    arr.dedup();

    bricks
        .iter()
        .map(|&(l, r)| {
            let left = arr.binary_search(&l).unwrap();
            let right = arr.binary_search(&r).unwrap();
            (left, right)
        })
        .collect()
}
