use itertools::Itertools;
use proconio::marker::Usize1;
use proconio::*;

#[fastout]
fn main() {
    input! {
        n: usize, m: usize,
        edges: [(Usize1, Usize1); m],
    }

    let mut adjacency_matrix = vec![vec![false; n]; n];
    for (u, v) in edges {
        adjacency_matrix[u][v] = true;
        adjacency_matrix[v][u] = true;
    }

    let mut count = 0;
    for comb in (0..n).combinations(3) {
        if adjacency_matrix[comb[0]][comb[1]]
            && adjacency_matrix[comb[1]][comb[2]]
            && adjacency_matrix[comb[2]][comb[0]]
        {
            count += 1;
        }
    }

    println!("{}", count);
}
