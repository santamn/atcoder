use proconio::*;
use std::cmp::{min, Reverse};
use std::collections::{BinaryHeap, HashMap};

fn main() {
    input! {q: usize}

    let mut map = HashMap::new();
    let mut maxheap = BinaryHeap::new();
    let mut minheap = BinaryHeap::new();

    for _ in 0..q {
        input! {kind: usize}

        match kind {
            1 => {
                input! {x: usize}
                *map.entry(x).or_insert(0) += 1;
                if map[&x] == 1 {
                    maxheap.push(x);
                    minheap.push(Reverse(x));
                }
            }
            2 => {
                input! {
                    x: usize, c:usize
                }

                if map.contains_key(&x) {
                    *map.get_mut(&x).unwrap() -= min(c, map[&x]);
                }
            }
            3 => {
                let mut max;
                let mut min;
                loop {
                    let &m = maxheap.peek().unwrap();
                    if map[&m] > 0 {
                        max = m;
                        break;
                    } else {
                        maxheap.pop();
                    }
                }

                loop {
                    let &Reverse(m) = minheap.peek().unwrap();
                    if map[&m] > 0 {
                        min = m;
                        break;
                    } else {
                        minheap.pop();
                    }
                }

                println!("{}", max - min);
            }
            _ => return,
        }
    }
}
