use num::integer;
use petgraph::algo::kosaraju_scc;
use petgraph::graph::Graph;
use proconio::*;

fn main() {
    input! {
        _: u32, m: u32,
        edges: [(u32, u32); m],
    }

    let g = Graph::<(), ()>::from_edges(&edges);
    let sccs = kosaraju_scc(&g);

    println!(
        "{}",
        sccs.iter().map(|scc| integer::binomial(scc.len(), 2)).sum()
    );
}
