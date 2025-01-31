use proconio::marker::Usize1;
use proconio::*;

type UnGraph = Vec<Vec<usize>>;

fn ungraph_from_edges(nodes: usize, edges: &Vec<(usize, usize)>) -> UnGraph {
    let mut ungraph = vec![Vec::new(); nodes];
    for (u, v) in edges {
        ungraph[*u].push(*v);
        ungraph[*v].push(*u);
    }
    ungraph
}

fn main() {
    input! {
        n: usize,
        edges: [(Usize1, Usize1); n-1],
    }
    // 木ではノードと孫ノードは隣り合わないので、自分自身と孫ノードを出力すればよい

    let graph = ungraph_from_edges(n, &edges);
    let nodes = dfs(&graph, 0, n);

    for &i in nodes.iter().take(n / 2) {
        print!("{} ", i)
    }
}

fn dfs(graph: &UnGraph, start: usize, n: usize) -> Vec<usize> {
    let mut color = vec![false; n];
    color[start] = true;

    let mut visited = vec![false; n];
    visited[start] = true;
    let mut stack = vec![start];
    while let Some(u) = stack.pop() {
        for &v in &graph[u] {
            if visited[v] {
                continue;
            }

            stack.push(v);
            visited[v] = true;
            color[v] = !color[u];
        }
    }

    let mut t = Vec::new();
    let mut f = Vec::new();
    for (i, &c) in color.iter().enumerate() {
        if c {
            t.push(i + 1);
        } else {
            f.push(i + 1);
        }
    }

    if t.len() > f.len() {
        t
    } else {
        f
    }
}
