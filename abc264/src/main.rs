use petgraph::unionfind::UnionFind;
use proconio::marker::Usize1;
use proconio::*;
use std::{cmp::min, collections::VecDeque};

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

    let mut uf = UnionFind::<usize>::new(n + 1);
    for (i, (u, v)) in wires.iter().enumerate() {
        if connected[i] {
            connect(&mut uf, *u, *v, n);
        }
    }

    let mut answer = VecDeque::new();
    for &i in x.iter().rev() {
        // FIXME: ここでO(|E|*|N|)になっている
        answer.push_front(count(&uf, n, n));
        connect(&mut uf, wires[i].0, wires[i].1, n);
    }

    for c in answer {
        println!("{}", c)
    }
}

fn connect(uf: &mut UnionFind<usize>, u: usize, v: usize, plant: usize) {
    uf.union(min(u, plant), min(v, plant));
}

fn count(uf: &UnionFind<usize>, plant: usize, points: usize) -> usize {
    (0..points).filter(|&k| uf.equiv(k, plant)).count()
}
