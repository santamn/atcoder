use proconio::{marker::Usize1, *};

type UnGraph = Vec<Vec<usize>>;

fn ungraph_from_edges(nodes: usize, edges: &[(usize, usize)]) -> UnGraph {
    let mut ungraph = vec![Vec::new(); nodes];
    for (u, v) in edges {
        ungraph[*u].push(*v);
        ungraph[*v].push(*u);
    }
    ungraph
}

#[fastout]
fn main() {
    input! {
        n: usize, m: usize,
        connections: [(Usize1, Usize1); m],
    }

    let graph = ungraph_from_edges(n, &connections);
}
