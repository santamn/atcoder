use petgraph::algo::dijkstra;
use petgraph::graph::UnGraph;
use proconio::*;

fn main() {
    input! {
        n: u32, m: u32,
        edges: [(u32, u32, u32); m]
    }

    let g = UnGraph::<(), u32>::from_edges(&edges);
    let dist_from_s = dijkstra(&g, 1.into(), None, |edge| *edge.weight());
    let dist_from_t = dijkstra(&g, n.into(), None, |edge| *edge.weight());

    for i in 1..=n {
        println!("{}", dist_from_s[&i.into()] + dist_from_t[&i.into()])
    }
}
