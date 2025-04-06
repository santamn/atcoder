use petgraph::{algo::dijkstra, graph::DiGraph};
use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};
use std::sync::OnceLock;

static H: OnceLock<usize> = OnceLock::new();
static W: OnceLock<usize> = OnceLock::new();

#[fastout]
fn main() {
    input! {
        h: usize, w: usize,
        s: [Chars; h],
        a: Usize1, b: Usize1,
        c: Usize1, d: Usize1,
    }

    H.set(h).unwrap();
    W.set(w).unwrap();

    let cost = s
        .iter()
        .flat_map(|row| row.iter().map(|&c| if c == '.' { 0 } else { 1 }))
        .collect::<Vec<usize>>();
    let graph = DiGraph::<(), usize, usize>::from_edges(
        (0..h * w).flat_map(|base| edges_4dirs(&cost, base)),
    );

    let distance = dijkstra(
        &graph,
        index_of(a, b).into(),
        Some(index_of(c, d).into()),
        |e| *e.weight(),
    );

    println!("{}", distance[&index_of(c, d).into()]);
}

fn edges_4dirs(cost: &[usize], base: usize) -> impl Iterator<Item = (usize, usize, usize)> + '_ {
    let (i, j) = index_of_2d(base);
    // 縦方向
    (i.saturating_sub(2)..=(i + 2).min(*H.get().unwrap() - 1))
        .filter_map(move |v| {
            let to = index_of(v, j);
            match v.abs_diff(i) {
                0 => None,
                1 => Some((base, to, cost[to])),
                2 => Some((base, to, cost[to] | cost[index_of((v + i) / 2, j)])),
                _ => unreachable!(),
            }
        })
        .chain(
            // 横方向
            (j.saturating_sub(2)..=(j + 2).min(*W.get().unwrap() - 1)).filter_map(move |h| {
                let to = index_of(i, h);
                match h.abs_diff(j) {
                    0 => None,
                    1 => Some((base, to, cost[to])),
                    2 => Some((base, to, cost[to] | cost[index_of(i, (h + j) / 2)])),
                    _ => unreachable!(),
                }
            }),
        )
}

fn index_of(i: usize, j: usize) -> usize {
    i * W.get().unwrap() + j
}

fn index_of_2d(base: usize) -> (usize, usize) {
    let w = *W.get().unwrap();
    (base / w, base % w)
}
