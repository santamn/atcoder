use petgraph::unionfind::UnionFind;
use proconio::{marker::Usize1, *};

#[fastout]
fn main() {
    input! {
        h: usize, w: usize,
        q: usize
    }

    let mut tiles = vec![vec![false; w]; h];
    let mut union_find = UnionFind::<usize>::new(h * w);

    for _ in 0..q {
        input! {p: usize}

        if p == 1 {
            input! {r: Usize1, c: Usize1}
            tiles[r][c] = true;
            if r > 0 && tiles[r - 1][c] {
                // 上
                union_find.union(r * w + c, (r - 1) * w + c);
            }
            if r + 1 < h && tiles[r + 1][c] {
                // 下
                union_find.union(r * w + c, (r + 1) * w + c);
            }
            if c > 0 && tiles[r][c - 1] {
                // 左
                union_find.union(r * w + c, r * w + c - 1);
            }
            if c + 1 < w && tiles[r][c + 1] {
                // 右
                union_find.union(r * w + c, r * w + c + 1);
            }
        } else {
            input! {a: Usize1, b: Usize1, c: Usize1, d: Usize1}
            println!(
                "{}",
                if tiles[a][b] && tiles[c][d] && union_find.equiv(a * w + b, c * w + d) {
                    "Yes"
                } else {
                    "No"
                }
            )
        }
    }
}
