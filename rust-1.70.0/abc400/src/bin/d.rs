use itertools::Itertools;
use petgraph::{algo::dijkstra, graph::DiGraph};
use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};

#[fastout]
fn main() {
    input! {
        h: usize, w: usize,
        s: [Chars; h],
        a: Usize1, b: Usize1,
        c: Usize1, d: Usize1,
    }

    let graph = DiGraph::<(), usize, usize>::from_edges(
        (0..h)
            .cartesian_product(0..w)
            .flat_map(|base| edges_4dir(&s, base, h, w)),
    );

    let hashmap = dijkstra(
        &graph,
        index_of((a, b), w).into(),
        Some(index_of((c, d), w).into()),
        |e| *e.weight(),
    );

    println!("{}", hashmap[&index_of((c, d), w).into()]);
}

fn edges_4dir(
    s: &[Vec<char>],
    base: (usize, usize),
    h: usize,
    w: usize,
) -> impl Iterator<Item = (usize, usize, usize)> {
    let (i, j) = base;

    let up = (i + 1).min(h)..(i + 3).min(h);
    let down = (i.saturating_sub(2)..i).rev();
    let left = (j.saturating_sub(2)..j).rev();
    let right = (j + 1).min(w)..(j + 3).min(w);

    let up_edges = up
        .clone()
        .map(move |x| (x, j))
        .zip(weights(&up.map(|x| s[x][j]).collect::<Vec<_>>()));
    let down_edges = down
        .clone()
        .map(move |x| (x, j))
        .zip(weights(&down.map(|x| s[x][j]).collect::<Vec<_>>()));
    let left_edges = left
        .clone()
        .map(move |y| (i, y))
        .zip(weights(&left.map(|y| s[i][y]).collect::<Vec<_>>()));
    let right_edges = right
        .clone()
        .map(move |y| (i, y))
        .zip(weights(&right.map(|y| s[i][y]).collect::<Vec<_>>()));

    up_edges
        .chain(down_edges)
        .chain(left_edges)
        .chain(right_edges)
        .map(move |(pos, weight)| (index_of(base, w), index_of(pos, w), weight))
}

fn weights(v: &[char]) -> Vec<usize> {
    match v {
        [] => vec![],
        ['.'] => vec![0],
        ['#'] => vec![1],
        ['.', '.'] => vec![0, 0],
        ['.', '#'] => vec![0, 1],
        _ => vec![1, 1],
    }
}

fn index_of(pos: (usize, usize), w: usize) -> usize {
    pos.0 * w + pos.1
}
