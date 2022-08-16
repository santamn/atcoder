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

fn dfs(nodes: usize, s: usize, t: usize, graph: &UnGraph) -> usize {
    let mut count = 0;
    let mut stack = vec![s];
    let mut visited = vec![false; nodes];

    while let Some(v) = stack.pop() {
        if !visited[v] {
            visited[v] = true;
            count += 1;
            if v == t {
                return count - 1;
            }

            for &w in &graph[v] {
                stack.push(w);
            }
        }
    }

    0
}

#[fastout]
fn main() {
    input! {
        n: usize,
        p: [Usize1; n-1]
    }

    let edges: Vec<_> = p.iter().enumerate().map(|(i, &p)| (i + 1, p)).collect();
    let graph = ungraph_from_edges(n, &edges);

    println!("{}", dfs(n, 0, n - 1, &graph))
}
