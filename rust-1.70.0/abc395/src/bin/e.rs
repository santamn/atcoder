use petgraph::algo::dijkstra;
use petgraph::graph::DiGraph;
use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n: usize, m: usize, x: usize,
        edges: [(Usize1, Usize1); m],
    }

    let graph = DiGraph::<(), usize, usize>::from_edges(
        edges
            .iter()
            .flat_map(|&(u, v)| [(u, v, 1), (v + n, u + n, 1)])
            .chain((0..n).flat_map(|i| [(i, i + n, x), (i + n, i, x)]))
            .chain([(n - 1, 2 * n, 0), (2 * n - 1, 2 * n, 0)]),
    );

    let dist = dijkstra(&graph, 0.into(), Some((2 * n).into()), |e| *e.weight());
    println!("{}", dist[&(2 * n).into()]);
}
