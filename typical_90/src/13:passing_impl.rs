use proconio::{marker::Usize1, *};
use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    position: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // PartialEqとの一貫性を持たせる必要がある
        // そのためcostが等しい場合にはpositionで比較する
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct Edge {
    to: usize,
    cost: usize,
}

fn dijkstra(adj_list: &Vec<Vec<Edge>>, start: usize) -> Vec<usize> {
    let mut dist = vec![std::usize::MAX; adj_list.len()];
    let mut heap = BinaryHeap::new();

    dist[start] = 0;
    heap.push(State {
        position: start,
        cost: 0,
    });

    while let Some(State { cost, position }) = heap.pop() {
        // 既に良い値が得られる場合はスキップ
        // この条件にかかる場合、それ以外の経路で既により良いものが存在する
        if cost > dist[position] {
            continue;
        }

        for edge in &adj_list[position] {
            let next = State {
                cost: edge.cost + dist[position],
                position: edge.to,
            };

            if next.cost < dist[edge.to] {
                dist[edge.to] = next.cost;
                heap.push(next);
            }
        }
    }

    dist
}

#[fastout]
fn main() {
    input! {
        n: usize, m: usize,
        edges: [(Usize1, Usize1, usize); m]
    }

    let mut adj_list: Vec<Vec<_>> = vec![Vec::new(); n];
    for edge in edges {
        adj_list[edge.0].push(Edge {
            to: edge.1,
            cost: edge.2,
        });
        adj_list[edge.1].push(Edge {
            to: edge.0,
            cost: edge.2,
        });
    }

    let dist1 = dijkstra(&adj_list, 0);
    let dist2 = dijkstra(&adj_list, n - 1);

    for i in 0..n {
        println!("{}", dist1[i] + dist2[i]);
    }
}
