use proconio::marker::Usize1;
use proconio::*;

type Graph = Vec<Vec<usize>>;

fn graph_from_edges(nodes: usize, edges: &Vec<(usize, usize)>) -> Graph {
    let mut graph = vec![Vec::new(); nodes];
    for (u, v) in edges {
        graph[*u].push(*v);
    }
    graph
}

fn main() {
    input! {
        n: usize, m: usize,
        edges: [(Usize1, Usize1); m],
    }

    let graph = graph_from_edges(n, &edges);
    let mut flags = vec![false; n];
    let mut dp = vec![0usize; n];
    for i in 0..n {
        length(&graph, &mut flags, &mut dp, i);
    }

    println!("{}", dp.iter().max().unwrap());
}

fn length(graph: &Graph, flags: &mut Vec<bool>, memo: &mut Vec<usize>, node: usize) -> usize {
    if flags[node] {
        return memo[node];
    } else {
        flags[node] = true;

        memo[node] = graph[node]
            .iter()
            .map(|&v| length(graph, flags, memo, v) + 1)
            .max()
            .unwrap_or_default();

        return memo[node];
    }
}
