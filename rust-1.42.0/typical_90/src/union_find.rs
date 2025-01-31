use proconio::*;

// https://atcoder.jp/contests/atc001/tasks/unionfind_a

struct UnionFind {
    par: Vec<usize>,  // 各要素の根
    rank: Vec<usize>, // 自分が所属する木のノードの数
}

impl UnionFind {
    fn new(n: usize) -> Self {
        UnionFind {
            par: (0..n).collect(),
            rank: vec![1; n],
        }
    }

    fn root(&mut self, x: usize) -> usize {
        if self.par[x] == x {
            return x;
        }
        // 経路圧縮
        self.par[x] = self.root(self.par[x]);
        self.par[x]
    }

    fn is_same(&mut self, x: usize, y: usize) -> bool {
        self.root(x) == self.root(y)
    }

    fn unite(&mut self, x: usize, y: usize) {
        let mut x = self.root(x);
        let mut y = self.root(y);

        if x == y {
            return;
        }

        if self.rank[x] < self.rank[y] {
            self.par[x] = y;
        } else {
            self.par[y] = x;

            if self.rank[x] == self.rank[y] {
                // 同じ高さの木をつなげるときにのみ木の高さは増える
                self.rank[x] += 1;
            }
        }
    }
}

#[fastout]
fn main() {
    input! {
        n: usize, q: usize
    }

    let mut union_find = UnionFind::new(n);

    for _ in 0..q {
        input! {
            p: usize,
            a: usize,
            b: usize
        }

        if p == 0 {
            union_find.unite(a, b)
        } else {
            println!(
                "{}",
                if union_find.is_same(a, b) {
                    "Yes"
                } else {
                    "No"
                }
            )
        }
    }
}
