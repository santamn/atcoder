use itertools::Itertools;
use proconio::*;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        dice: [[usize]; n],
    }

    println!(
        "{}",
        dice.into_iter()
            .map(|v| {
                let n = v.len() as f64;
                v.into_iter().fold(HashMap::new(), |mut prob, num| {
                    *prob.entry(num).or_insert(0.0) += 1.0 / n;
                    prob
                })
            })
            .tuple_combinations()
            .map(|(a, b)| {
                a.iter()
                    .map(|(key, value1)| b.get(key).map(|value2| value1 * value2).unwrap_or(0.0))
                    .sum::<f64>()
            })
            .reduce(f64::max)
            .unwrap()
    );
}
