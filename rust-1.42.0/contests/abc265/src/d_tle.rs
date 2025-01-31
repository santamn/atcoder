use itertools::Itertools;
use proconio::*;
use std::collections::HashMap;
use std::collections::VecDeque;

fn main() {
    input! {
        n: usize, p: usize, q: usize, r: usize,
        a: [usize; n],
    }

    let mut cumsum: VecDeque<usize> = a
        .iter()
        .scan(0, |cum, x| {
            *cum += x;
            Some(*cum)
        })
        .collect();
    cumsum.push_front(0);

    let map: HashMap<_, _> = cumsum
        .iter()
        .enumerate()
        .map(|(i, &x)| (x + p, i))
        .chain(cumsum.iter().enumerate().map(|(i, &x)| (x, i)))
        .into_group_map();

    for v in map
        .iter()
        .filter(|&(_, v)| v.len() >= 2)
        .map(|(_, v)| v.clone())
    {
        if let Ok(z) = find_threshold_index(&cumsum, v[1], q) {
            if let Ok(_) = find_threshold_index(&cumsum, z, r) {
                println!("Yes");
                return;
            }
        }
    }
    println!("No");
}

fn find_threshold_index(cumsum: &VecDeque<usize>, start: usize, value: usize) -> Result<usize, ()> {
    for i in start + 1..cumsum.len() {
        if cumsum[i] - cumsum[start] == value {
            return Ok(i);
        }
    }

    Err(())
}
