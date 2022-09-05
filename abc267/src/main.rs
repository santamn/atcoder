use proconio::marker::Usize1;
use proconio::*;
use std::cmp::max;
use std::cmp::Ordering;
use std::collections::BinaryHeap;

type UnGraph = Vec<Vec<usize>>;

fn ungraph_from_edges(nodes: usize, edges: &Vec<(usize, usize)>) -> UnGraph {
    let mut ungraph = vec![Vec::new(); nodes];
    for (u, v) in edges {
        ungraph[*u].push(*v);
        ungraph[*v].push(*u);
    }
    ungraph
}

#[derive(Debug, Clone, Copy, Eq)]
struct Node {
    id: usize,
    cost: usize,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        self.cost.cmp(&other.cost).reverse()
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost
    }
}

fn main() {
    input! {
        n: usize, m: usize,
        a: [usize; n],
        edges: [(Usize1, Usize1); m],
    }

    let mut costs = vec![0; n];
    for &(u, v) in &edges {
        costs[u] += a[v];
        costs[v] += a[u];
    }

    let graph = ungraph_from_edges(n, &edges);
    let mut eliminated = vec![false; n];
    let mut heap: BinaryHeap<Node> = costs
        .iter()
        .enumerate()
        .map(|(id, &cost)| Node { id, cost })
        .collect();

    let mut answer = 0;
    while let Some(node) = heap.pop() {
        if eliminated[node.id] {
            continue;
        }

        for &v in &graph[node.id] {
            costs[v] -= a[node.id];
            heap.push(Node {
                id: v,
                cost: costs[v],
            });
        }

        answer = max(answer, node.cost);
        eliminated[node.id] = true;
    }

    println!("{}", answer);
}
