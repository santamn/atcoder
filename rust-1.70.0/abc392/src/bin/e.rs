use petgraph::unionfind::UnionFind;
use proconio::{fastout, input, marker::Usize1};
use std::collections::HashSet;

#[fastout]
fn main() {
    input! {
        n: usize, m: usize,
        cables: [(Usize1, Usize1); m],
    }

    let mut uf: UnionFind<usize> = UnionFind::new(n);
    let mut redundant_edges = Vec::new();
    for (i, (u, v)) in cables.into_iter().enumerate() {
        if uf.equiv(u, v) {
            redundant_edges.push((i, u, v));
        } else {
            uf.union(u, v);
        }
    }

    let mut roots = uf
        .clone()
        .into_labeling()
        .into_iter()
        .collect::<HashSet<_>>();

    println!("{}", roots.len() - 1);
    for &(i, u, v) in &redundant_edges {
        if let Some(&root) = roots.iter().find(|&&root| !uf.equiv(root, u)) {
            println!("{} {} {}", i + 1, v + 1, root + 1);
            uf.union(root, u);
            roots.remove(&root);
        }
    }
}
