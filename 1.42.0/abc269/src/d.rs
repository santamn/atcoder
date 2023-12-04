use itertools::Itertools;
use proconio::*;
use std::collections::HashMap;

struct UnionFind {
    par: Vec<usize>,
    rank: Vec<usize>,
    siz: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        UnionFind {
            par: (0..n).collect(),
            rank: vec![1; n],
            siz: vec![1; n],
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

    fn into_labeling(&mut self) -> Vec<usize> {
        (0..self.par.len()).map(|x| self.root(x)).collect_vec()
    }

    fn is_same(&mut self, x: usize, y: usize) -> bool {
        self.root(x) == self.root(y)
    }

    fn unite(&mut self, x: usize, y: usize) {
        let x = self.root(x);
        let y = self.root(y);

        if x == y {
            return;
        }

        let (small, tall) = if self.rank[x] < self.rank[y] {
            (x, y)
        } else {
            (y, x)
        };
        self.par[small] = tall;
        self.siz[tall] += self.siz[small];

        if self.rank[small] == self.rank[tall] {
            self.rank[tall] += 1;
        }
    }

    fn size(&mut self, x: usize) -> usize {
        let root = self.root(x);
        self.siz[root]
    }
}

fn main() {
    input! {
        n: usize,
        coordinates: [(isize, isize); n],
    }

    let hashmap: HashMap<(isize, isize), usize> = coordinates
        .iter()
        .enumerate()
        .map(|(i, &k)| (k, i))
        .collect();

    let mut uf = UnionFind::new(n);
    for (i, &(x, y)) in coordinates.iter().enumerate() {
        if let Some(&j) = hashmap.get(&(x - 1, y - 1)) {
            uf.unite(i, j);
        }
        if let Some(&j) = hashmap.get(&(x - 1, y)) {
            uf.unite(i, j);
        }
        if let Some(&j) = hashmap.get(&(x, y - 1)) {
            uf.unite(i, j);
        }
        if let Some(&j) = hashmap.get(&(x, y + 1)) {
            uf.unite(i, j);
        }
        if let Some(&j) = hashmap.get(&(x + 1, y)) {
            uf.unite(i, j);
        }
        if let Some(&j) = hashmap.get(&(x + 1, y + 1)) {
            uf.unite(i, j);
        }
    }

    let mut v = uf.into_labeling();
    v.sort();
    v.dedup();
    println!("{}", v.len());
}
