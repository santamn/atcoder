use proconio::*;

type Graph = Vec<Vec<usize>>;

fn main() {
    input! {
        n: usize,
        mut edges: [(usize, usize); n-1],
    }

    let mut graph: Graph = vec![Vec::new(); n];
    for (u, v) in edges {
        graph[u - 1].push(v - 1);
        graph[v - 1].push(u - 1);
    }

    // 頂点0から距離最大の点を調べる
    let t = dfs(&graph, 0)
        .iter()
        .enumerate()
        .fold((0, 0), |(max_idx, max), (idx, val)| {
            if max > *val {
                (max_idx, max)
            } else {
                (idx, *val)
            }
        })
        .0;

    // 頂点tから距離最大の点までの距離を求める
    let dist = dfs(&graph, t);
    let diameter = dist.iter().max().unwrap();

    println!("{}", diameter + 1)
}

// 頂点sから各頂点への距離を求める
fn dfs(graph: &Graph, s: usize) -> Vec<usize> {
    let mut dist: Vec<Option<usize>> = vec![None; graph.len()];
    dist[s] = Some(0);

    let mut stack = vec![s];
    while !stack.is_empty() {
        let v = stack.pop().unwrap();
        for u in &graph[v] {
            if dist[*u] == None {
                stack.push(*u);
                dist[*u] = Some(dist[v].unwrap() + 1);
            }
        }
    }

    dist.iter().map(|v| v.unwrap()).collect()
}
