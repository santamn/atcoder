use proconio::marker::Usize1;
use proconio::*;
use std::{cmp::min, collections::VecDeque};

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

    fn is_same(&mut self, x: usize, y: usize) -> bool {
        self.root(x) == self.root(y)
    }

    fn unite(&mut self, x: usize, y: usize) {
        let mut x = self.root(x);
        let mut y = self.root(y);

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

#[fastout]
fn main() {
    input! {
        n: usize, _: usize, e: usize,
        wires: [(Usize1, Usize1); e],
        q: usize,
        x: [Usize1; q],
    }

    let mut connected = vec![true; e];
    for &i in &x {
        connected[i] = false;
    }

    let mut uf = UnionFind::new(n + 1);
    for (i, (u, v)) in wires.iter().enumerate() {
        if connected[i] {
            connect(&mut uf, *u, *v, n);
        }
    }

    let mut answer = VecDeque::new();
    for &i in x.iter().rev() {
        answer.push_front(uf.size(n) - 1);
        connect(&mut uf, wires[i].0, wires[i].1, n);
    }

    for c in answer {
        println!("{}", c)
    }
}

fn connect(uf: &mut UnionFind, u: usize, v: usize, plant: usize) {
    uf.unite(min(u, plant), min(v, plant));
}
