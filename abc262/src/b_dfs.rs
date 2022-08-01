use proconio::marker::Usize1;
use proconio::*;
use std::collections::HashSet;

// このコードは正しく動作しない
// DFSではこの問題を解くことができない

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
        n: usize, m: usize,
        edges: [(Usize1, Usize1); m],
    }

    let graph = ungraph_from_edges(n, &edges);
    let trianges: HashSet<(usize, usize, usize)> =
        (0..n).map(|u| dfs(n, u, &graph)).flatten().collect();
    println!("{:?}", trianges);
}

// 深さ優先探索で三角形を探す
fn dfs(nodes: usize, u: usize, graph: &UnGraph) -> Vec<(usize, usize, usize)> {
    struct S {
        id: usize,
        path: Vec<usize>,
    }

    let mut triangles = Vec::new();
    let mut stack = vec![S {
        id: u,
        path: vec![u],
    }];
    let mut visited = vec![false; nodes];
    while let Some(S { id: v, path: p }) = stack.pop() {
        if !visited[v] {
            visited[v] = true;

            for &w in &graph[v] {
                if p.len() == 3 {
                    if w == u {
                        // 三角形を発見
                        let mut q = p.clone();
                        q.sort();
                        triangles.push((q[0], q[1], q[2]));
                    }
                    continue;
                } else {
                    let mut q = p.clone();
                    q.push(w);
                    stack.push(S { id: w, path: q })
                }
            }
        }
    }

    triangles
}
